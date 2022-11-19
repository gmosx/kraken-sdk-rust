use serde::Serialize;
use crate::client::Request;

/// Cancels all pending orders.
/// <https://docs.kraken.com/websockets-v2/#cancel-all-orders>
#[derive(Debug, Serialize)]
pub struct CancelAllOrdersRequest<'a> {
    /// Session token.
    pub token: &'a str,
}

impl Request for CancelAllOrdersRequest<'_> {
    fn method(&self) -> &'static str {
        "cancel_all"
    }
}

impl CancelAllOrdersRequest<'_> {
    pub fn new(token: &str) -> CancelAllOrdersRequest {
        CancelAllOrdersRequest { token }
    }
}