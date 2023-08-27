//! <https://docs.kraken.com/websockets-v2/#batch-cancel>

use crate::client::{PrivateParams, PrivateRequest, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct BatchCancelParams {
    /// Array of strings representing either: order_userref(s) or order_id(s)
    /// Note: array must contain between 2 and 50 items.
    pub orders: Vec<String>,
}

/// Multiple orders can be canceled in one request via batch_cancel method.
///
/// <https://docs.kraken.com/websockets-v2/#batch-cancel>
pub type BatchCancelRequest = PrivateRequest<BatchCancelParams>;

impl BatchCancelRequest {
    pub fn new(orders: Vec<String>) -> Self {
        Self {
            method: "batch_cancel".to_owned(),
            params: PrivateParams::new(BatchCancelParams { orders }),
            req_id: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BatchCancelResult {
    pub count: i32,
}

pub type BatchCancelResponse = Response<BatchCancelResult>;
