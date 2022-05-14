use super::models;
use crate::Gateway;
use common::{create_config, Context, InnerMessage};
use serde_json::{from_value, Value};
use slog::{info, warn};

pub struct Binance {
    context: Context<InnerMessage>,
}

impl Default for Binance {
    fn default() -> Self {
        let mut cfg = create_config()
            .set_default(
                "binance_spot_ws_url",
                "wss://stream.binance.com:9443/stream",
            )
            .expect("set default binance ws URL")
            .build()
            .expect("binance config");
        Self {
            context: Context::new("binance", Some(cfg)),
        }
    }
}

impl Gateway for Binance {
    fn name(&self) -> &str {
        &self.context.name
    }

    fn subscribe(&self, symbols: Vec<String>) {}

    fn unsubscribe(&self, symbols: Vec<String>) {}

    fn on_websocket_message(&self, value: Value) {
        let result: Result<models::WsResponse, serde_json::Error> = from_value(value.clone());
        match result {
            Ok(o) => match o {
                models::WsResponse::Subscriptions(sub) => {
                    info!(self.context.logger, "{:?}", sub.channels);
                }
                models::WsResponse::Heartbeat(_) => {}
                models::WsResponse::Snapshot(ref book) => {
                    self.book_snapshot(book);
                }
                models::WsResponse::L2update(ref book) => {
                    self.book_update(book);
                }
            },
            Err(err) => {
                warn!(self.context.logger, "{}. {}", err, value);
            }
        };
    }
}

impl Binance {
    fn book_snapshot(&self, book: &models::Book) {}

    fn book_update(&self, book: &models::BookUpdate) {}
}
