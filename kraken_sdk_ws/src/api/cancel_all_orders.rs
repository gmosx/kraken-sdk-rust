use serde::Serialize;
use crate::client::{Request};

#[derive(Debug, Serialize)]
pub struct CancelAllOrdersParams<'a> {
    /// Session token.
    pub token: &'a str,
}

/// Cancels all pending orders.
///
/// <https://docs.kraken.com/websockets-v2/#cancel-all-orders>
pub type CancelAllOrdersRequest<'a> = Request<CancelAllOrdersParams<'a>>;

impl CancelAllOrdersRequest<'_> {
    pub fn new(token: &str) -> CancelAllOrdersRequest {
        CancelAllOrdersRequest {
            method: "subscribe".to_owned(),
            params: CancelAllOrdersParams { token },
            req_id: None,
        }
    }
}