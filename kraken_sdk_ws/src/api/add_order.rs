use serde::Serialize;
use crate::{client::Request, types::{OrderType, OrderSide, TimeInForce}};

/// <https://docs.kraken.com/websockets-v2/#add-order>
#[derive(Debug, Serialize)]
pub struct AddOrderRequest<'a> {
    pub token: &'a str,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_qty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_userref: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
}

impl Request for AddOrderRequest<'_> {
    fn method(&self) -> &'static str {
        "add_order"
    }
}

impl AddOrderRequest<'_> {
    pub fn market<'a>(side: OrderSide, order_qty: f64, symbol: &'a str, token: &'a str) -> AddOrderRequest<'a> {
        AddOrderRequest {
            side,
            limit_price: None,
            order_qty: Some(order_qty),
            order_type: OrderType::Limit,
            symbol,
            time_in_force: None,
            order_userref: None,
            post_only: None,
            validate: None,
            token,
        }
    }

    pub fn limit<'a>(side: OrderSide, order_qty: f64, symbol: &'a str, limit_price: f64, token: &'a str) -> AddOrderRequest<'a> {
        AddOrderRequest {
            side,
            limit_price: Some(limit_price),
            order_qty: Some(order_qty),
            order_type: OrderType::Limit,
            symbol,
            time_in_force: None,
            order_userref: None,
            post_only: None,
            validate: None,
            token,
        }
    }

    pub fn buy_limit<'a>(order_qty: f64, symbol: &'a str, limit_price: f64, token: &'a str) -> AddOrderRequest<'a> {
        AddOrderRequest::limit(OrderSide::Buy, order_qty, symbol, limit_price,  token)
    }

    pub fn sell_limit<'a>(order_qty: f64, symbol: &'a str, limit_price: f64, token: &'a str) -> AddOrderRequest<'a> {
        AddOrderRequest::limit(OrderSide::Sell, order_qty, symbol, limit_price,  token)
    }

    pub fn post_only(self, post_only: bool) -> Self {
        Self {
            post_only: Some(post_only),
            ..self
        }
    }

    pub fn validate(self, validate: bool) -> Self {
        Self {
            validate: Some(validate),
            ..self
        }
    }

    pub fn validate_only(self) -> Self {
        Self {
            validate: Some(true),
            ..self
        }
    }


}