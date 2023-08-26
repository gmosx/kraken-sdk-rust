//! <https://docs.kraken.com/websockets-v2/#cancel-order>

use serde::Serialize;

use crate::client::{PrivateParams, PrivateRequest};

use super::CANCEL_ORDER_METHOD;

/// Note: Though order_id and order_userref are individually optional, at least
/// one of them must be filled.
#[derive(Debug, Serialize)]
pub struct CancelOrderParams {
    /// Array of strings representing order_id(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<Vec<String>>,
    /// Array of strings representing order_userref(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_userref: Option<Vec<i32>>,
}

impl CancelOrderParams {
    pub fn order_id(order_id: impl Into<String>) -> Self {
        let order_id = vec![order_id.into()];
        Self {
            order_id: Some(order_id),
            order_userref: None,
        }
    }

    pub fn order_ids(order_id: Vec<String>) -> Self {
        Self {
            order_id: Some(order_id),
            order_userref: None,
        }
    }

    pub fn order_userref(order_userref: i32) -> Self {
        let order_userref = vec![order_userref];
        Self {
            order_id: None,
            order_userref: Some(order_userref),
        }
    }

    pub fn order_userrefs(order_userref: Vec<i32>) -> Self {
        Self {
            order_id: None,
            order_userref: Some(order_userref),
        }
    }
}

pub type CancelOrderRequest = PrivateRequest<CancelOrderParams>;

impl CancelOrderRequest {
    pub fn new(params: CancelOrderParams) -> Self {
        Self {
            method: CANCEL_ORDER_METHOD.into(),
            params: PrivateParams::new(params),
            req_id: None,
        }
    }
}
