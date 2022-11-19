use serde::{Serialize, Deserialize};
use crate::client::{IRequest, Response};

/// Multiple orders can be canceled in one request via batch_cancel method.
///
/// <https://docs.kraken.com/websockets-v2/#batch-cancel>
#[derive(Debug, Serialize)]
pub struct BatchCancelRequest<'a> {
    /// Session token.
    pub token: &'a str,
    /// Array of strings representing either: order_userref(s) or order_id(s)
    /// Note: array must contain between 2 and 50 items.
    pub orders: Vec<String>,
}

impl IRequest for BatchCancelRequest<'_> {
    fn method(&self) -> &'static str {
        "batch_cancel"
    }
}

impl BatchCancelRequest<'_> {
    pub fn new(orders: Vec<String>, token: &str) -> BatchCancelRequest {
        BatchCancelRequest { token, orders }
    }
}

#[derive(Debug, Deserialize)]
pub struct BatchCancelResult {
    pub count: i32,
}

pub type BatchCancelResponse = Response<BatchCancelResult>;
