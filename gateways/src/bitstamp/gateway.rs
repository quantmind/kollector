use super::models;
use crate::{Gateway, WsUpdate};
use common::{Book, Context, InnerMessage, WsConsumer};
use serde_json::{from_value, Value};
use slog::{info, warn};

/// Bitstamp Gateway
///
/// This gatewat stream order book snapshots from
/// [bitstamp websocket api](https://www.bitstamp.net/websocket/v2/)
pub struct Bitstamp {
    context: Context<InnerMessage>,
    ws: WsConsumer,
    max_depth: usize,
}

impl Gateway for Bitstamp {
    fn name(&self) -> &str {
        &self.context.name
    }

    fn ws_consumer(&self) -> WsConsumer {
        self.ws.clone()
    }

    fn subscribe(&mut self, symbols: &[String]) {
        for symbol in symbols.iter() {
            self.ws
                .write(models::Command::subscribe("order_book", symbol));
        }
    }

    fn unsubscribe(&mut self, symbols: &[String]) {
        for symbol in symbols.iter() {
            self.ws
                .write(models::Command::unsubscribe("order_book", symbol));
        }
    }

    fn on_websocket_message(&mut self, value: Value) -> Option<WsUpdate> {
        let result: Result<models::WsResponse, serde_json::Error> = from_value(value.clone());
        match result {
            Ok(o) => match o {
                models::WsResponse::Subscriptions(sub) => {
                    info!(self.context.logger, "{:?}", sub);
                }
                models::WsResponse::Book(ref book) => {
                    return self.book_snapshot(book);
                }
            },
            Err(err) => {
                warn!(self.context.logger, "{}. {}", err, value);
            }
        };
        None
    }
}

impl Bitstamp {
    pub fn new(context: &Context<InnerMessage>, max_depth: usize) -> Self {
        let mut context = context.clone();
        context.name = "bitstamp".to_owned();
        let ws_url = context
            .cfg
            .get("bitstamp_ws_url")
            .unwrap_or_else(|_| "wss://ws.bitstamp.net");
        let ws = WsConsumer::new(&context, ws_url);
        Self {
            context,
            ws,
            max_depth,
        }
    }

    fn book_snapshot(&self, book: &models::Book) -> Option<WsUpdate> {
        let mut ob = Book::new(&book.channel.split("_").last().unwrap().to_lowercase());
        ob.asks.update(&book.data.asks);
        ob.bids.update(&book.data.bids);
        Some(WsUpdate::Book(ob))
    }
}
