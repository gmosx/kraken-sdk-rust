use crate::{
    client::{PrivateParams, PrivateRequest},
    types::{OrderSide, OrderType, TimeInForce},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AddOrderParams {
    pub side: OrderSide,
    pub order_type: OrderType,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<f64>,
    /// Order quantity in terms of the base asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_qty: Option<f64>,
    /// When set this turns the order into an iceberg order with display_qty as
    /// visible quantity and hiding rest of order_qty. This can only be used
    /// with limit order type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_qty: Option<f64>,
    /// RFC3339 timestamp (e.g. 2021-04-01T00:18:45Z) of scheduled start time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// RFC3339 timestamp (e.g. 2021-04-01T00:18:45Z) of expiration time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_userref: Option<i32>,
    /// Disable market price protection for market orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_mpp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
}

/// - <https://docs.kraken.com/websockets-v2/#add-order>
pub type AddOrderRequest = PrivateRequest<AddOrderParams>;

impl AddOrderRequest {
    pub fn market(side: OrderSide, order_qty: f64, symbol: impl Into<String>) -> Self {
        AddOrderRequest {
            method: "add_order".to_owned(),
            params: PrivateParams::new(AddOrderParams {
                side,
                limit_price: None,
                order_qty: Some(order_qty),
                display_qty: None,
                effective_time: None,
                expire_time: None,
                order_type: OrderType::Limit,
                symbol: symbol.into(),
                time_in_force: None,
                order_userref: None,
                no_mpp: None,
                post_only: None,
                reduce_only: None,
                validate: None,
            }),
            req_id: None,
        }
    }

    pub fn limit(
        side: OrderSide,
        order_qty: f64,
        symbol: impl Into<String>,
        limit_price: f64,
    ) -> Self {
        Self {
            method: "add_order".to_owned(),
            params: PrivateParams::new(AddOrderParams {
                side,
                limit_price: Some(limit_price),
                order_qty: Some(order_qty),
                display_qty: None,
                effective_time: None,
                expire_time: None,
                order_type: OrderType::Limit,
                symbol: symbol.into(),
                time_in_force: None,
                order_userref: None,
                no_mpp: None,
                post_only: None,
                reduce_only: None,
                validate: None,
            }),
            req_id: None,
        }
    }

    pub fn buy_limit(order_qty: f64, symbol: impl Into<String>, limit_price: f64) -> Self {
        AddOrderRequest::limit(OrderSide::Buy, order_qty, symbol, limit_price)
    }

    pub fn sell_limit(order_qty: f64, symbol: impl Into<String>, limit_price: f64) -> Self {
        AddOrderRequest::limit(OrderSide::Sell, order_qty, symbol, limit_price)
    }

    pub fn display_qty(self, display_qty: f64) -> Self {
        Self {
            params: PrivateParams {
                params: AddOrderParams {
                    display_qty: Some(display_qty),
                    ..self.params.params
                },
                ..self.params
            },
            ..self
        }
    }

    pub fn effective_time(self, effective_time: String) -> Self {
        Self {
            params: PrivateParams {
                params: AddOrderParams {
                    effective_time: Some(effective_time),
                    ..self.params.params
                },
                ..self.params
            },
            ..self
        }
    }

    pub fn expire_time(self, expire_time: String) -> Self {
        Self {
            params: PrivateParams {
                params: AddOrderParams {
                    expire_time: Some(expire_time),
                    ..self.params.params
                },
                ..self.params
            },
            ..self
        }
    }

    pub fn no_mpp(self, no_mpp: bool) -> Self {
        Self {
            params: PrivateParams {
                params: AddOrderParams {
                    no_mpp: Some(no_mpp),
                    ..self.params.params
                },
                ..self.params
            },
            ..self
        }
    }

    pub fn post_only(self, post_only: bool) -> Self {
        Self {
            params: PrivateParams {
                params: AddOrderParams {
                    post_only: Some(post_only),
                    ..self.params.params
                },
                ..self.params
            },
            ..self
        }
    }

    pub fn reduce_only(self, reduce_only: bool) -> Self {
        Self {
            params: PrivateParams {
                params: AddOrderParams {
                    reduce_only: Some(reduce_only),
                    ..self.params.params
                },
                ..self.params
            },
            ..self
        }
    }

    pub fn validate_only(self, validate: bool) -> Self {
        Self {
            params: PrivateParams {
                params: AddOrderParams {
                    validate: Some(validate),
                    ..self.params.params
                },
                ..self.params
            },
            ..self
        }
    }
}
