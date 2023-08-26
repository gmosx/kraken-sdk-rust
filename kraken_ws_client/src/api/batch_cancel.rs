use crate::client::{PublicRequest, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct BatchCancelParams<'a> {
    /// Session token.
    pub token: &'a str,
    /// Array of strings representing either: order_userref(s) or order_id(s)
    /// Note: array must contain between 2 and 50 items.
    pub orders: Vec<String>,
}

/// Multiple orders can be canceled in one request via batch_cancel method.
///
/// <https://docs.kraken.com/websockets-v2/#batch-cancel>
pub type BatchCancelRequest<'a> = PublicRequest<BatchCancelParams<'a>>;

impl BatchCancelRequest<'_> {
    pub fn new(orders: Vec<String>, token: &str) -> BatchCancelRequest {
        BatchCancelRequest {
            method: "batch_cancel".to_owned(),
            params: BatchCancelParams { token, orders },
            req_id: None,
        }
    }
}
#[derive(Debug, Deserialize)]
pub struct BatchCancelResult {
    pub count: i32,
}

pub type BatchCancelResponse = Response<BatchCancelResult>;
