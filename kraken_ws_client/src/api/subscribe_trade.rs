use serde::{Deserialize, Serialize};

use crate::{
    client::{Event, PublicRequest},
    types::{Channel, OrderSide, OrderType},
};

#[derive(Debug, Serialize)]
pub struct SubscribeTradeParams {
    pub channel: Channel,
    pub symbol: Vec<String>,
    /// Request a snapshot after subscribing, default=true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

pub type SubscribeTradeRequest = PublicRequest<SubscribeTradeParams>;

impl SubscribeTradeRequest {
    pub fn new(symbol: impl Into<Vec<String>>) -> Self {
        Self {
            method: "subscribe".into(),
            params: SubscribeTradeParams {
                channel: Channel::Trade,
                symbol: symbol.into(),
                snapshot: None,
            },
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
