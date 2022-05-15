use super::models;
use super::requests;
use crate::{Gateway, WsUpdate};
use async_channel::{unbounded, Receiver, Sender};
use common::{Book, BookSnapshot, Context, HttpClient, InnerMessage, WsConsumer};
use serde_json::{from_value, Value};
use slog::{info, warn};
use std::collections::HashMap;

struct BookWithBuffer {
    asset: String,
    buffer: Vec<models::BookUpdate>,
    book: Option<Book>,
}

/// Binance Gateway
///
/// The binance gateway requires to fetch order book snapshots via Rest
/// and maintain the book updates in memory
pub struct Binance {
    context: Context<InnerMessage>,
    ws: WsConsumer,
    max_depth: usize,
    msg_id: usize,
    books: HashMap<String, BookWithBuffer>,
    depth_request_sender: Sender<String>,
    depth_request_receiver: Receiver<String>,
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
                        models::WsData::BookUpdate(ref book) => {
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
        let api_url = context
            .cfg
            .get("binance_spot_url")
            .unwrap_or_else(|_| "https://api.binance.com");
        let ws = WsConsumer::new(&context, ws_url);
        let (depth_request_sender, depth_request_receiver) = unbounded();
        Self {
            context,
            ws,
            max_depth,
            msg_id: 0,
            books: HashMap::new(),
            depth_request_sender,
            depth_request_receiver,
        }
    }

    fn http(&self) -> HttpClient {
        let api_url = self
            .context
            .cfg
            .get("binance_spot_url")
            .unwrap_or_else(|_| "https://api.binance.com");
        HttpClient::new(api_url)
    }

    fn book_snapshot(&mut self, book: &models::BookUpdate) -> Option<WsUpdate> {
        let asset = book.s.to_lowercase();
        let asset_str = &asset;
        let b = self
            .books
            .entry(asset_str.to_owned())
            .or_insert_with(|| BookWithBuffer::new(asset_str));
        match &b.book {
            Some(book) => {
                // pass for now
            }
            None => {
                // fetch the book snapshot via rest
                let http = self.http();
                let context = self.context.clone();
                tokio::spawn(async move {
                    let result = http
                        .request(requests::GetDepth::new(&asset), Some(&context.logger))
                        .await;
                    match result {
                        Ok(b) => {
                            let mut book = Book::new(&asset);
                            book.asks.update(&b.asks);
                            book.bids.update(&b.bids);
                            context.send(InnerMessage::BookSnapshot(BookSnapshot {
                                name: context.name.to_owned(),
                                sequence: b.last_update_id,
                                book,
                            }));
                        }
                        Err(err) => {
                            // pass
                        }
                    }
                });
            }
        }
        None
    }
}

impl BookWithBuffer {
    fn new(asset: &str) -> Self {
        Self {
            asset: asset.to_owned(),
            buffer: vec![],
            book: None,
        }
    }
}
