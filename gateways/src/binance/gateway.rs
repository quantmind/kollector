use super::models;
use super::requests;
use crate::{Gateway, WsUpdate};
use common::{
    Book, BookSnapshot, HttpClient, InnerMessage, NewBookSnapshot, WorkerContext, WsConsumer,
};
use serde_json::{from_value, Value};
use slog::{error, info, warn};
use std::collections::HashMap;
use std::time::Duration;

struct BookWithBuffer {
    updates: Vec<models::BookUpdate>,
    book: Option<Book>,
}

/// Binance Gateway
///
/// The binance gateway requires to fetch order book snapshots via Rest
/// and maintain the book updates in memory with messages coming from the websocket api.
/// For more information check
/// [binance documentation](https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md)
pub struct Binance {
    context: WorkerContext,
    ws: WsConsumer,
    max_depth: usize,
    msg_id: usize,
    books: HashMap<String, BookWithBuffer>,
}

impl Gateway for Binance {
    fn name(&self) -> &str {
        &self.context.name
    }

    fn ws_consumer(&self) -> WsConsumer {
        self.ws.clone()
    }

    // request book snapshot at regular intervals
    // this is so we have a consistent book
    fn setup(&self) {
        let context = self.context.clone();
        let heartbeat_millis: u64 = context
            .get_or("binance_snapshot_heartbeat", 10000)
            .expect("websocket binance_snapshot_heartbeat");
        let mut delay = 2;
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(delay)).await;
                context
                    .send(InnerMessage::NewBookSnapshot(NewBookSnapshot {
                        name: context.name.to_owned(),
                    }))
                    .await;
                delay = heartbeat_millis;
            }
        });
    }

    fn subscribe(&mut self, symbols: &[String]) {
        self.msg_id += 1;
        self.ws.write(models::WsMessage::subscribe(
            self.msg_id,
            "depth@100ms",
            symbols,
        ));
    }

    fn unsubscribe(&mut self, symbols: &[String]) {
        self.msg_id += 1;
        self.ws.write(models::WsMessage::unsubscribe(
            self.msg_id,
            "depth@100ms",
            symbols,
        ));
    }

    fn request_snapshot(&mut self) {
        let mut assets = vec![];
        for (asset, bf) in self.books.iter_mut() {
            assets.push(asset.to_owned());
            bf.reset();
        }
        fetch_snapshots(assets, self.http(), self.context.clone());
    }

    fn on_websocket_message(&mut self, value: Value) -> Option<WsUpdate> {
        let result: Result<models::WsResponse, serde_json::Error> = from_value(value.clone());
        match result {
            Ok(o) => {
                if let Some(data) = o.data {
                    match data {
                        models::WsData::BookUpdate(book) => {
                            return self.book_snapshot(book);
                        }
                    }
                };
            }
            Err(err) => {
                warn!(self.context.logger, "{}. {}", err, value);
            }
        }
        None
    }

    fn on_book_snapshot(&mut self, snapshot: BookSnapshot) -> Option<Book> {
        match self.books.get_mut(&snapshot.book.asset) {
            Some(bf) => {
                info!(
                    self.context.logger,
                    "{} received an orderbook snapshot {}", snapshot.name, snapshot.book.asset
                );
                return bf.on_book_snapshot(&snapshot, self.max_depth);
            }
            None => {
                warn!(
                    self.context.logger,
                    "received an unknown orderbook snapshot {:?}", snapshot
                );
            }
        }
        None
    }
}

impl Binance {
    /// Create a new Binance gateway
    pub fn new(context: &WorkerContext, max_depth: usize, _pairs: &[String]) -> Self {
        let mut context = context.clone();
        context.name = "binance".to_owned();
        let ws_url: &str = context
            .get_or(
                "binance_spot_ws_url",
                "wss://stream.binance.com:9443/stream",
            )
            .expect("Binance websocket url");
        let ws = WsConsumer::new(&context, ws_url);
        Self {
            context,
            ws,
            max_depth,
            msg_id: 0,
            books: HashMap::new(),
        }
    }

    // Http client
    fn http(&self) -> HttpClient {
        let api_url: &str = self
            .context
            .get_or("binance_spot_url", "https://api.binance.com")
            .expect("Binance api url");
        HttpClient::new(api_url)
    }

    fn book_snapshot(&mut self, book_update: models::BookUpdate) -> Option<WsUpdate> {
        let asset = book_update.s.to_lowercase();
        let bf = self.books.entry(asset).or_insert_with(BookWithBuffer::new);
        match &mut bf.book {
            Some(book) => {
                book.asks.update(&book_update.a);
                book.bids.update(&book_update.b);
                Some(WsUpdate::Book(book.trim(self.max_depth)))
            }
            None => {
                // push the update in the buffer
                bf.updates.push(book_update);
                None
            }
        }
    }
}

fn fetch_snapshots(assets: Vec<String>, http: HttpClient, context: WorkerContext) {
    tokio::spawn(async move {
        for asset in assets.iter() {
            let request = requests::GetDepth::new(asset, 1000);
            info!(
                context.logger,
                "{} fetching orderbook {} snapshot via rest", context.name, asset
            );
            let result = http.request(request, Some(&context.logger)).await;
            match result {
                Ok(b) => {
                    let mut book = Book::new(asset);
                    book.asks.update(&b.asks);
                    book.bids.update(&b.bids);
                    context
                        .send(InnerMessage::BookSnapshot(BookSnapshot {
                            name: context.name.to_owned(),
                            sequence: b.last_update_id,
                            book,
                        }))
                        .await;
                }
                Err(err) => {
                    error!(
                        context.logger,
                        "{} - unexpected error - {}", context.name, err
                    );
                    context.send(InnerMessage::Failure).await;
                }
            }
        }
    });
}

impl BookWithBuffer {
    fn new() -> Self {
        Self {
            updates: vec![],
            book: None,
        }
    }

    fn reset(&mut self) {
        self.book = None;
    }

    fn on_book_snapshot(&mut self, snapshot: &BookSnapshot, max_depth: usize) -> Option<Book> {
        let mut book = snapshot.book.clone();
        for update in self.updates.iter() {
            if update.u > snapshot.sequence {
                book.asks.update(&update.a);
                book.bids.update(&update.b);
            }
        }
        self.updates = vec![];
        let ob = book.trim(max_depth);
        self.book = Some(book);
        Some(ob)
    }
}
