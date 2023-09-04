use serde::{Deserialize, Serialize};

use crate::{
    client::{Event, PrivateParams, PrivateRequest},
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

// #todo consider renaming this to ExecutionsSubscription?

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
pub struct Execution {
    pub cost: Option<f64>,
    pub exec_id: Option<String>,
    pub exec_type: String,
    pub fees: Option<Vec<Amount>>,
    pub liquidity_ind: Option<String>,
    pub order_type: Option<OrderType>,
    pub order_id: String,
    pub order_status: OrderStatus,
    pub order_userref: Option<u32>,
    pub avg_price: Option<f64>,
    pub last_price: Option<f64>,
    pub limit_price: Option<f64>,
    pub stop_price: Option<f64>,
    pub triggered_price: Option<f64>,
    pub order_qty: Option<f64>,
    pub side: Option<OrderSide>,
    pub symbol: Option<String>,
    pub timestamp: String,
    pub trade_id: Option<i64>,
}

pub type ExecutionData = Vec<Execution>;

pub type ExecutionsEvent = Event<Vec<ExecutionData>>;
