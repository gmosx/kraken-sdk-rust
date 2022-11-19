use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::{client::{Request, Event}, types::SubscriptionName};

/// - <https://docs.kraken.com/websockets-v2/#ticker>
#[derive(Debug, Serialize)]
pub struct SubscribeTickerRequest<'a> {
    pub channel: SubscriptionName,
    pub symbol: &'a [&'a str],
    /// Request a snapshot after subscribing.
    /// Default: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

impl Request for SubscribeTickerRequest<'_> {
    fn method(&self) -> &'static str {
        "subscribe"
    }
}

impl SubscribeTickerRequest<'_> {
    pub fn new<'a>(symbol: &'a[&'a str]) -> SubscribeTickerRequest<'a> {
        SubscribeTickerRequest {
            channel: SubscriptionName::Ticker,
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
pub struct TickerData {
    pub ask: Decimal,
    pub ask_qty: Decimal,
    pub bid: Decimal,
    pub bid_qty: Decimal,
    pub change: Decimal,
    pub change_pct: Decimal,
    pub high: Decimal,
    pub last: Decimal,
    pub low: Decimal,
    pub symbol: String,
    pub volume: Decimal,
    pub vwap: Decimal,
}

pub type TickerEvent = Event<Vec<TickerData>>;