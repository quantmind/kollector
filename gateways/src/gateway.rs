use common::{Book, BookSnapshot, WsConsumer};
use serde_json::Value;

/// A websocket update
///
/// This can be extended to handle other type of updates
pub enum WsUpdate {
    Book(Book),
}

/// A Gateway trait
pub trait Gateway {
    fn setup(&self) {}
    //
    fn name(&self) -> &str;

    // subscribe to a set of assets
    fn subscribe(&mut self, symbols: &[String]);

    // unsubscribe from a set of assets
    fn unsubscribe(&mut self, symbols: &[String]);

    // handle a websocket message from exchange
    fn on_websocket_message(&mut self, value: Value) -> Option<WsUpdate>;

    // handle a book snapshot
    fn on_book_snapshot(&mut self, _snapshot: BookSnapshot) -> Option<Book> {
        None
    }

    fn ws_consumer(&self) -> WsConsumer;
}
