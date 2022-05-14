use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct WsMessage {
    pub id: usize,
    pub method: String,
    pub params: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    filter_type: String,
    min_qty: Option<f32>,
    max_qty: Option<f32>,
    step_size: Option<f32>,
    min_price: Option<f32>,
    max_price: Option<f32>,
    tick_size: Option<f32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotInstrument {
    pub symbol: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub filters: Vec<Filter>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureInstrument {
    pub symbol: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub status: String,
    pub contract_type: String,
    pub maint_margin_percent: String,
    pub required_margin_percent: String,
    pub delivery_date: Option<String>,
    pub filters: Vec<Filter>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpotInfo {
    #[serde(rename = "symbols")]
    pub assets: Vec<SpotInstrument>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FutureInfo {
    pub assets: Vec<FutureInstrument>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelProducts {
    pub name: String,
    pub product_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Heartbeat {
    pub product_id: String,
    pub sequence: u64,
    pub last_trade_id: u64,
    pub time: String,
}

#[derive(Debug, Clone, Deserialize)]
pub enum Channel {
    Name(String),
    Products(ChannelProducts),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Channels {
    pub channels: Vec<ChannelProducts>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Book {
    pub product_id: String,
    pub bids: Vec<(String, String)>,
    pub asks: Vec<(String, String)>,
    pub time: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BookUpdate {
    pub product_id: String,
    pub time: Option<String>,
    pub changes: Vec<(String, String, String)>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsResponse {
    Subscriptions(Channels),
    Heartbeat(Heartbeat),
    Snapshot(Book),
    L2update(BookUpdate),
}
