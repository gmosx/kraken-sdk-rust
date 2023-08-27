//! <https://docs.kraken.com/websockets-v2/#cancel-order>

use serde::Serialize;

use crate::client::{PrivateParams, PrivateRequest};

use super::CANCEL_ORDER_METHOD;

/// Even though order_id and order_userref are individually optional, at least
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

/// ### Example
/// ```rs
/// let req = CancelOrderRequest::order_id("...");
/// client.send(req).await?;
/// ```
pub type CancelOrderRequest = PrivateRequest<CancelOrderParams>;

impl CancelOrderRequest {
    pub fn new(order_id: Option<Vec<String>>, order_userref: Option<Vec<i32>>) -> Self {
        Self {
            method: CANCEL_ORDER_METHOD.into(),
            params: PrivateParams::new(CancelOrderParams {
                order_id,
                order_userref,
            }),
            req_id: None,
        }
    }

    pub fn order_id(order_id: impl Into<String>) -> Self {
        let order_id = vec![order_id.into()];
        Self::new(Some(order_id), None)
    }

    pub fn order_ids(order_id: Vec<String>) -> Self {
        Self::new(Some(order_id), None)
    }

    pub fn order_userref(order_userref: i32) -> Self {
        let order_userref = vec![order_userref];
        Self::new(None, Some(order_userref))
    }

    pub fn order_userrefs(order_userref: Vec<i32>) -> Self {
        Self::new(None, Some(order_userref))
    }
}
