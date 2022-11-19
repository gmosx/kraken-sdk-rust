use serde::{Deserialize, Serialize};
use crate::{client::IRequest, types::{OrderType, OrderStatus, Amount, OrderSide, SubscriptionName}};

/// - <https://docs.kraken.com/websockets-v2/#executions>
/// - <https://docs.kraken.com/websockets/#message-ownTrades>
/// - <https://docs.kraken.com/websockets/#message-openOrders>
#[derive(Debug, Serialize)]
pub struct SubscribeExecutionsRequest<'a> {
    pub channel: SubscriptionName,
    /// Request a snapshot of the last 50 execution events.
    /// Default: true
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
    pub token: &'a str,
}

impl IRequest for SubscribeExecutionsRequest<'_> {
    fn method(&self) -> &'static str {
        "subscribe"
    }
}

impl SubscribeExecutionsRequest<'_> {
    pub fn new(token: &str) -> SubscribeExecutionsRequest<'_> {
        SubscribeExecutionsRequest {
            channel: SubscriptionName::Executions,
            snapshot: None,
            snapshot_trades: None,
            order_status: None,
            ratecounter: None,
            token,
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
