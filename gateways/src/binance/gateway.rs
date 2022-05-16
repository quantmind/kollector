use super::models;
use super::requests;
use crate::{Gateway, WsUpdate};
use common::{Book, BookSnapshot, Context, HttpClient, InnerMessage, WsConsumer};
use serde_json::{from_value, Value};
use slog::warn;
use std::collections::HashMap;

struct BookWithBuffer {
    updates: Vec<models::BookUpdate>,
    book: Option<Book>,
}

/// Binance Gateway
///
/// The binance gateway requires to fetch order book snapshots via Rest
/// and maintain the book updates in memory with messages coming from the websocket api.
/// Fr more information check
/// [binance documentation](https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md)
pub struct Binance {
    context: Context<InnerMessage>,
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

    fn on_websocket_message(&mut self, value: Value) -> Option<WsUpdate> {
        let result: Result<models::WsResponse, serde_json::Error> = from_value(value.clone());
        match result {
            Ok(o) => {
                if let Some(data) = o.data {
                    match data {
                        models::WsData::BookUpdate(book) => {
                            return self.book_snapshot(book);
                        }
                        _ => {
                            warn!(self.context.logger, "{}", value);
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
        match self.books.get_mut(&snapshot.name) {
            Some(bf) => {
                return bf.on_book_snapshot(&snapshot);
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
    pub fn new(context: &Context<InnerMessage>, max_depth: usize) -> Self {
        let mut context = context.clone();
        context.name = "binance".to_owned();
        let ws_url = context
            .cfg
            .get("binance_spot_ws_url")
            .unwrap_or_else(|_| "wss://stream.binance.com:9443/stream");
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
        let api_url = self
            .context
            .cfg
            .get("binance_spot_url")
            .unwrap_or_else(|_| "https://api.binance.com");
        HttpClient::new(api_url)
    }

    fn book_snapshot(&mut self, book_update: models::BookUpdate) -> Option<WsUpdate> {
        let asset = book_update.s.to_lowercase();
        let bf = self
            .books
            .entry(asset.to_owned())
            .or_insert_with(|| BookWithBuffer::new());
        match &mut bf.book {
            Some(book) => {
                book.asks.update(&book_update.a);
                book.bids.update(&book_update.b);
                Some(WsUpdate::Book(book.clone()))
            }
            None => {
                // push the update in the buffer
                bf.updates.push(book_update);
                // fetch the book snapshot via rest
                let http = self.http();
                let context = self.context.clone();
                let request = requests::GetDepth::new(&asset, self.max_depth);
                tokio::spawn(async move {
                    let result = http.request(request, Some(&context.logger)).await;
                    match result {
                        Ok(b) => {
                            let mut book = Book::new(&asset);
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
                            // pass
                        }
                    }
                });
                None
            }
        }
    }
}

impl BookWithBuffer {
    fn new() -> Self {
        Self {
            updates: vec![],
            book: None,
        }
    }

    fn on_book_snapshot(&mut self, snapshot: &BookSnapshot) -> Option<Book> {
        let mut book = snapshot.book.clone();
        for update in self.updates.iter() {
            if update.u > snapshot.sequence {
                book.asks.update(&update.a);
                book.bids.update(&update.b);
            }
        }
        self.book = Some(book.clone());
        Some(book)
    }
}
