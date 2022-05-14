use serde_json::Value;

/// A Gateway trait
pub trait Gateway {
    //
    fn name(&self) -> &str;

    // subscribe to a set of assets
    fn subscribe(&self, symbols: Vec<String>);

    // unsubscribe from a set of assets
    fn unsubscribe(&self, symbols: Vec<String>);

    // handle a websocket message from exchange
    fn on_websocket_message(&self, value: Value);
}
