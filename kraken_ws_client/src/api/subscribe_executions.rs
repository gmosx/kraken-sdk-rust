use crate::{
    client::Request,
    types::{Amount, Channel, OrderSide, OrderStatus, OrderType}, util::gen_next_id,
};
use serde::{Deserialize, Serialize};

use super::SUBSCRIBE_METHOD;

#[derive(Debug, Serialize)]
pub struct SubscribeExecutionsParams {
    pub channel: Channel,
    /// Request a snapshot after subscribing, default=true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
    /// If true snapshot only provides execution events referencing trades.
    /// Default: false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_trades: Option<bool>,
    /// If true client will receive a message for all possible execution status
    /// transitions. If false client will only receive updates when order
    /// enters the book (new) and when order is closed (filled, canceled or
    /// expired).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratecounter: Option<bool>,
    pub token: String,
}

impl SubscribeExecutionsParams {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            channel: Channel::Executions,
            snapshot: None,
            snapshot_trades: None,
            order_status: None,
            ratecounter: None,
            token: token.into(),
        }
    }

    pub fn snapshot(self, snapshot: bool) -> Self {
        Self {
            snapshot: Some(snapshot),
            ..self
        }
    }

    pub fn ratecounter(self, ratecounter: bool) -> Self {
        Self {
            ratecounter: Some(ratecounter),
            ..self
        }
    }
}

/// - <https://docs.kraken.com/websockets-v2/#executions>
/// - <https://docs.kraken.com/websockets/#message-ownTrades>
/// - <https://docs.kraken.com/websockets/#message-openOrders>
pub type SubscribeExecutionsRequest = Request<SubscribeExecutionsParams>;

impl SubscribeExecutionsRequest {
    pub fn new(params: SubscribeExecutionsParams) -> Self {
        Self {
            method: SUBSCRIBE_METHOD.into(),
            params,
            req_id: Some(gen_next_id()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ExecutionData {
    pub cost: i64,
    pub exec_id: String,
    pub exec_type: String,
    pub fees: Vec<Amount>,
    pub liquidity_ind: String,
    pub order_type: OrderType,
    pub order_id: String,
    pub order_status: OrderStatus,
    pub order_userref: String,
    pub price: f64,
    pub order_qty: f64,
    pub side: OrderSide,
    pub symbol: String,
    pub timestamp: String,
    pub trade_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct ExecutionsEvent {
    pub channel: String,
    pub data: Vec<ExecutionData>,
    pub sequence: i32,
    #[serde(rename = "type")]
    pub event_type: String,
}
