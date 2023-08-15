use crate::{
    client::Request,
    types::{OrderSide, OrderType, TimeInForce},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AddOrderParams<'a> {
    pub token: &'a str,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub symbol: &'a str,
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
pub type AddOrderRequest<'a> = Request<AddOrderParams<'a>>;

impl AddOrderRequest<'_> {
    pub fn market<'a>(
        side: OrderSide,
        order_qty: f64,
        symbol: &'a str,
        token: &'a str,
    ) -> AddOrderRequest<'a> {
        AddOrderRequest {
            method: "add_order".to_owned(),
            params: AddOrderParams {
                side,
                limit_price: None,
                order_qty: Some(order_qty),
                display_qty: None,
                order_type: OrderType::Limit,
                symbol,
                time_in_force: None,
                order_userref: None,
                no_mpp: None,
                post_only: None,
                reduce_only: None,
                validate: None,
                token,
            },
            req_id: None,
        }
    }

    pub fn limit<'a>(
        side: OrderSide,
        order_qty: f64,
        symbol: &'a str,
        limit_price: f64,
        token: &'a str,
    ) -> AddOrderRequest<'a> {
        AddOrderRequest {
            method: "add_order".to_owned(),
            params: AddOrderParams {
                side,
                limit_price: Some(limit_price),
                order_qty: Some(order_qty),
                display_qty: None,
                order_type: OrderType::Limit,
                symbol,
                time_in_force: None,
                order_userref: None,
                no_mpp: None,
                post_only: None,
                reduce_only: None,
                validate: None,
                token,
            },
            req_id: None,
        }
    }

    pub fn buy_limit<'a>(
        order_qty: f64,
        symbol: &'a str,
        limit_price: f64,
        token: &'a str,
    ) -> AddOrderRequest<'a> {
        AddOrderRequest::limit(OrderSide::Buy, order_qty, symbol, limit_price, token)
    }

    pub fn sell_limit<'a>(
        order_qty: f64,
        symbol: &'a str,
        limit_price: f64,
        token: &'a str,
    ) -> AddOrderRequest<'a> {
        AddOrderRequest::limit(OrderSide::Sell, order_qty, symbol, limit_price, token)
    }

    pub fn display_qty(self, display_qty: f64) -> Self {
        Self {
            params: AddOrderParams {
                display_qty: Some(display_qty),
                ..self.params
            },
            ..self
        }
    }

    pub fn no_mpp(self, no_mpp: bool) -> Self {
        Self {
            params: AddOrderParams {
                no_mpp: Some(no_mpp),
                ..self.params
            },
            ..self
        }
    }

    pub fn post_only(self, post_only: bool) -> Self {
        Self {
            params: AddOrderParams {
                post_only: Some(post_only),
                ..self.params
            },
            ..self
        }
    }

    pub fn reduce_only(self, reduce_only: bool) -> Self {
        Self {
            params: AddOrderParams {
                reduce_only: Some(reduce_only),
                ..self.params
            },
            ..self
        }
    }

    pub fn validate(self, validate: bool) -> Self {
        Self {
            params: AddOrderParams {
                validate: Some(validate),
                ..self.params
            },
            ..self
        }
    }

    // #TODO find a better name.
    pub fn validate_only(self) -> Self {
        Self {
            params: AddOrderParams {
                validate: Some(true),
                ..self.params
            },
            ..self
        }
    }
}
