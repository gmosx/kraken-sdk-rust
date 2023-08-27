use serde::{Deserialize, Serialize};

use crate::{
    client::{PrivateParams, PrivateRequest},
    types::{Amount, Channel, OrderSide, OrderStatus, OrderType},
};

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
}

/// - <https://docs.kraken.com/websockets-v2/#executions>
/// - <https://docs.kraken.com/websockets/#message-ownTrades>
/// - <https://docs.kraken.com/websockets/#message-openOrders>
pub type SubscribeExecutionsRequest = PrivateRequest<SubscribeExecutionsParams>;

impl SubscribeExecutionsRequest {
    pub fn new() -> Self {
        Self {
            method: "subscribe".into(),
            params: PrivateParams::new(SubscribeExecutionsParams {
                channel: Channel::Executions,
                snapshot: None,
                snapshot_trades: None,
                order_status: None,
                ratecounter: None,
            }),
            req_id: None,
        }
    }

    pub fn snapshot(self, snapshot: bool) -> Self {
        Self {
            params: PrivateParams {
                params: SubscribeExecutionsParams {
                    snapshot: Some(snapshot),
                    ..self.params.params
                },
                ..self.params
            },
            ..self
        }
    }

    pub fn ratecounter(self, ratecounter: bool) -> Self {
        Self {
            params: PrivateParams {
                params: SubscribeExecutionsParams {
                    ratecounter: Some(ratecounter),
                    ..self.params.params
                },
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
