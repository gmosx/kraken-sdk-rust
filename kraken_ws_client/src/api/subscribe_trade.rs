use serde::{Deserialize, Serialize};

use crate::{
    client::{Event, PublicRequest},
    types::{Channel, OrderSide, OrderType},
    util::gen_next_id,
};

use super::SUBSCRIBE_METHOD;

#[derive(Debug, Serialize)]
pub struct SubscribeTradeParams {
    pub channel: Channel,
    pub symbol: Vec<String>,
    /// Request a snapshot after subscribing, default=true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

impl SubscribeTradeParams {
    pub fn new<'a>(symbol: impl Into<Vec<String>>) -> Self {
        Self {
            channel: Channel::Trade,
            symbol: symbol.into(),
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

pub type SubscribeTradeRequest = PublicRequest<SubscribeTradeParams>;

impl SubscribeTradeRequest {
    pub fn new(params: SubscribeTradeParams) -> Self {
        Self {
            method: SUBSCRIBE_METHOD.into(),
            params,
            req_id: Some(gen_next_id()),
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
