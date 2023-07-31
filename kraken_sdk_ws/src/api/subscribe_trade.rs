use serde::{Deserialize, Serialize};
use crate::{client::{Event, Request}, types::{OrderSide, OrderType, Channel}};

#[derive(Debug, Serialize)]
pub struct SubscribeTradeParams<'a> {
    pub channel: Channel,
    pub symbol: &'a [&'a str],
    /// Request a snapshot after subscribing, default=true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

/// - <https://docs.kraken.com/websockets-v2/#trade>
pub type SubscribeTradeRequest<'a> = Request<SubscribeTradeParams<'a>>;

impl SubscribeTradeRequest<'_> {
    pub fn new<'a>(symbol: &'a[&'a str]) -> SubscribeTradeRequest<'a> {
        SubscribeTradeRequest {
            method: "subscribe".to_owned(),
            params: SubscribeTradeParams { channel:  Channel::Trade, symbol, snapshot: None },
            req_id: None,
        }
    }

    pub fn snapshot(self, snapshot: bool) -> Self {
        Self {
            params: SubscribeTradeParams {
                snapshot: Some(snapshot),
                ..self.params
            },
            ..self
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TradeData {
    pub ord_type: OrderType,
    pub price: f64,
    pub qty: f64,
    pub side: OrderSide,
    pub symbol: String,
    pub timestamp: String,
    pub trade_id: i64,
}

pub type TradeEvent = Event<Vec<TradeData>>;