use serde::{Deserialize, Serialize};
use crate::{client::{Request, Event}, types::SubscriptionName};

/// - <https://docs.kraken.com/websockets-v2/#open-high-low-and-close-ohlc>
#[derive(Debug, Serialize)]
pub struct SubscribeOhlcRequest<'a> {
    pub channel: SubscriptionName,
    pub symbol: &'a [&'a str],
    /// Request a snapshot after subscribing.
    /// Default: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

impl Request for SubscribeOhlcRequest<'_> {
    fn method(&self) -> &'static str {
        "subscribe"
    }
}

impl SubscribeOhlcRequest<'_> {
    pub fn new<'a>(symbol: &'a[&'a str]) -> SubscribeOhlcRequest<'a> {
        SubscribeOhlcRequest {
            channel: SubscriptionName::OHLC,
            symbol,
            snapshot: None,
        }
    }

    pub fn snapshot(self, snapshot: bool) -> Self {
        Self {
            snapshot: Some(snapshot),
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
