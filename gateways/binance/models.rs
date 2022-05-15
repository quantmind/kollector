use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct WsMessage {
    pub id: usize,
    pub method: String,
    pub params: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookSnapshot {
    pub last_update_id: usize,
    pub asks: Vec<(String, String)>,
    pub bids: Vec<(String, String)>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BookUpdate {
    pub s: String,
    pub a: Vec<(String, String)>,
    pub b: Vec<(String, String)>,
    #[serde(rename = "E")]
    pub time: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "e", rename_all = "snake_case")]
pub enum WsData {
    #[serde(rename = "depthUpdate")]
    BookUpdate(BookUpdate),
}

#[derive(Debug, Clone, Deserialize)]
pub struct WsResponse {
    pub data: Option<WsData>,
}

impl WsMessage {
    pub fn subscribe(id: usize, channel: &str, symbols: &[String]) -> Self {
        Self::op("SUBSCRIBE", id, channel, symbols)
    }

    pub fn unsubscribe(id: usize, channel: &str, symbols: &[String]) -> Self {
        Self::op("UNSUBSCRIBE", id, channel, symbols)
    }

    fn op(method: &str, id: usize, channel: &str, symbols: &[String]) -> Self {
        Self {
            id,
            method: method.to_owned(),
            params: symbols
                .iter()
                .map(|symbol| format!("{}@{}", symbol, channel))
                .collect(),
        }
    }
}
