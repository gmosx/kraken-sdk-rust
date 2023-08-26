//! <https://docs.kraken.com/websockets-v2/#cancel-order>

use crate::client::Request;
use serde::Serialize;

use super::CANCEL_ORDER_METHOD;

/// Note: Though order_id and order_userref are individually optional, at least
/// one of them must be filled.
#[derive(Debug, Serialize)]
pub struct CancelOrderParams<'a> {
    /// Session token.
    pub token: &'a str,
    /// Array of strings representing order_id(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<Vec<String>>,
    /// Array of strings representing order_userref(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_userref: Option<Vec<i32>>,
}

pub type CancelOrderRequest<'a> = Request<CancelOrderParams<'a>>;

impl CancelOrderRequest<'_> {
    pub fn order_id(order_id: impl Into<String>, token: &str) -> CancelOrderRequest {
        let order_id = vec![order_id.into()];
        CancelOrderRequest {
            method: CANCEL_ORDER_METHOD.to_owned(),
            params: CancelOrderParams {
                token,
                order_id: Some(order_id),
                order_userref: None,
            },
            req_id: None,
        }
    }

    pub fn order_ids(order_id: Vec<String>, token: &str) -> CancelOrderRequest {
        CancelOrderRequest {
            method: CANCEL_ORDER_METHOD.to_owned(),
            params: CancelOrderParams {
                token,
                order_id: Some(order_id),
                order_userref: None,
            },
            req_id: None,
        }
    }

    pub fn order_userref(order_userref: i32, token: &str) -> CancelOrderRequest {
        let order_userref = vec![order_userref];
        CancelOrderRequest {
            method: CANCEL_ORDER_METHOD.to_owned(),
            params: CancelOrderParams {
                token,
                order_id: None,
                order_userref: Some(order_userref),
            },
            req_id: None,
        }
    }

    pub fn order_userrefs(order_userref: Vec<i32>, token: &str) -> CancelOrderRequest {
        CancelOrderRequest {
            method: CANCEL_ORDER_METHOD.to_owned(),
            params: CancelOrderParams {
                token,
                order_id: None,
                order_userref: Some(order_userref),
            },
            req_id: None,
        }
    }
}

// #todo

// impl Client {
// cancel_order_id
// cancel_order_ids
// cancel_order_userref
// cancel_order_userrefs
// }

/*
    let cancel_order_req = CancelOrderRequest::order_id(..., client.token.unwrap()).extra(bool);
    let resp = client.send(cancel_order_req).await;

    let resp = client.cancel_order_id(...).
*/
