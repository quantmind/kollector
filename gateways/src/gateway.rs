use common::{Book, BookSnapshot, WsConsumer};
use serde_json::Value;

/// A websocket update
///
/// This can be extended to handle other type of updates
pub enum WsUpdate {
    Book(Book),
}

/// A Gateway trait
///
/// A gateway implements the middleware for connecting to remote crypto exchanges
/// and parse orderbook (or other) messages in a streaming fashion.
pub trait Gateway {
    fn setup(&self) {}

    /// Request book snapshots
    fn request_snapshot(&mut self) {}

    /// gateway name
    fn name(&self) -> &str;

    /// subscribe to a set of assets
    ///
    /// assets are lowercase, the gateway implementation is responsible
    /// for mapping names to the relevant exchange asset names
    fn subscribe(&mut self, symbols: &[String]);

    /// unsubscribe from a set of assets
    fn unsubscribe(&mut self, symbols: &[String]);

    /// handle a websocket message from exchange
    fn on_websocket_message(&mut self, value: Value) -> Option<WsUpdate>;

    /// handle a book snapshot
    fn on_book_snapshot(&mut self, _snapshot: BookSnapshot) -> Option<Book> {
        None
    }

    /// return the websocket consumer for this gateway
    fn ws_consumer(&self) -> WsConsumer;
}
