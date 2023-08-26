use crate::{
    client::Request,
    types::{Amount, Channel, OrderSide, OrderStatus, OrderType},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SubscribeExecutionsParams<'a> {
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
    pub token: &'a str,
}

/// - <https://docs.kraken.com/websockets-v2/#executions>
/// - <https://docs.kraken.com/websockets/#message-ownTrades>
/// - <https://docs.kraken.com/websockets/#message-openOrders>
pub type SubscribeExecutionsRequest<'a> = Request<SubscribeExecutionsParams<'a>>;

impl SubscribeExecutionsRequest<'_> {
    pub fn new(token: &str) -> SubscribeExecutionsRequest<'_> {
        SubscribeExecutionsRequest {
            method: "subscribe".to_owned(),
            params: SubscribeExecutionsParams {
                channel: Channel::Executions,
                snapshot: None,
                snapshot_trades: None,
                order_status: None,
                ratecounter: None,
                token,
            },
            req_id: None,
        }
    }

    pub fn snapshot(self, snapshot: bool) -> Self {
        Self {
            params: SubscribeExecutionsParams {
                snapshot: Some(snapshot),
                ..self.params
            },
            ..self
        }
    }

    pub fn ratecounter(self, ratecounter: bool) -> Self {
        Self {
            params: SubscribeExecutionsParams {
                ratecounter: Some(ratecounter),
                ..self.params
            },
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
