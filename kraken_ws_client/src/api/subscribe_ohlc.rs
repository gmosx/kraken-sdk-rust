use serde::{Deserialize, Serialize};

use crate::{
    client::{Event, PublicRequest},
    types::Channel,
};

#[derive(Debug, Serialize)]
pub struct SubscribeOhlcParams {
    pub channel: Channel,
    pub symbol: Vec<String>,
    /// Request a snapshot after subscribing, default=true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

/// - <https://docs.kraken.com/websockets-v2/#open-high-low-and-close-ohlc>
pub type SubscribeOhlcRequest = PublicRequest<SubscribeOhlcParams>;

impl SubscribeOhlcRequest {
    pub fn new(symbol: impl Into<Vec<String>>) -> Self {
        Self {
            method: "subscribe".into(),
            params: SubscribeOhlcParams {
                channel: Channel::OHLC,
                symbol: symbol.into(),
                snapshot: None,
            },
            req_id: None,
        }
    }

    pub fn snapshot(self, snapshot: bool) -> Self {
        Self {
            params: SubscribeOhlcParams {
                snapshot: Some(snapshot),
                ..self.params
            },
            ..self
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OhlcData {
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub symbol: String,
    pub timestamp: String,
    pub trades: i64,
    pub volume: f64,
    pub vwap: f64,
}

pub type OhlcEvent = Event<Vec<OhlcData>>;
