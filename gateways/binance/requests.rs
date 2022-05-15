use super::models;
use common::Request;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetDepth {
    symbol: String,
}

impl Request for GetDepth {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/api/v3/depth";
    const HAS_PAYLOAD: bool = true;
    type Response = models::BookSnapshot;
}

impl GetDepth {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
        }
    }
}
