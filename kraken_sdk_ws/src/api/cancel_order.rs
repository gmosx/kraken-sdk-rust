use serde::Serialize;
use crate::client::Request;

/// <https://docs.kraken.com/websockets-v2/#cancel-order>
///
/// Note: Though order_id and order_userref are individually optional, at least
/// one of them must be filled.
#[derive(Debug, Serialize)]
pub struct CancelOrderRequest<'a> {
    /// Session token.
    pub token: &'a str,
    /// Array of strings representing order_id(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<Vec<String>>,
    /// Array of strings representing order_userref(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_userref: Option<Vec<i32>>,
}

// #TODO add a check if the request is valid before sending.

impl Request for CancelOrderRequest<'_> {
    fn method(&self) -> &'static str {
        "cancel_order"
    }
}

impl CancelOrderRequest<'_> {
    pub fn order_id<'a>(order_id: &'a str, token: &'a str) -> CancelOrderRequest<'a> {
        let order_id = vec![order_id.to_owned()];
        CancelOrderRequest { token, order_id: Some(order_id), order_userref: None }
    }

    pub fn order_ids(order_id: Vec<String>, token: &str) -> CancelOrderRequest {
        CancelOrderRequest { token, order_id: Some(order_id), order_userref: None }
    }

    pub fn order_userref(order_userref: i32,token: &str) -> CancelOrderRequest {
        let order_userref = vec![order_userref];
        CancelOrderRequest { token, order_id: None, order_userref: Some(order_userref) }
    }

    pub fn order_userrefs(order_userref: Vec<i32>,token: &str) -> CancelOrderRequest {
        CancelOrderRequest { token, order_id: None, order_userref: Some(order_userref) }
    }
}