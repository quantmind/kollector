use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Event {
    pub event: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    channel: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Command {
    pub event: String,
    data: Channel,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BookData {
    pub asks: Vec<(String, String)>,
    pub bids: Vec<(String, String)>,
    pub microtimestamp: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Book {
    pub channel: String,
    pub data: BookData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BookUpdate {
    pub product_id: String,
    pub time: Option<String>,
    pub changes: Vec<(String, String, String)>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "event")]
pub enum WsResponse {
    #[serde(rename = "bts:subscription_succeeded")]
    Subscriptions(Channel),
    #[serde(rename = "data")]
    Book(Book),
}

impl Command {
    pub fn subscribe(channel: &str, pair: &str) -> Self {
        Self::op("bts:subscribe", channel, pair)
    }

    pub fn unsubscribe(channel: &str, pair: &str) -> Self {
        Self::op("bts:unsubscribe", channel, pair)
    }

    fn op(event: &str, channel: &str, pair: &str) -> Self {
        Self {
            event: event.to_owned(),
            data: Channel {
                channel: format!("{}_{}", channel, pair),
            },
        }
    }
}
