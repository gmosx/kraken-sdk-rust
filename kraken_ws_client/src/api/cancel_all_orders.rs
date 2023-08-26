use crate::client::PublicRequest;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CancelAllOrdersParams<'a> {
    /// Session token.
    pub token: &'a str,
}

/// Cancels all pending orders.
///
/// <https://docs.kraken.com/websockets-v2/#cancel-all-orders>
pub type CancelAllOrdersRequest<'a> = PublicRequest<CancelAllOrdersParams<'a>>;

impl CancelAllOrdersRequest<'_> {
    pub fn new(token: &str) -> CancelAllOrdersRequest {
        CancelAllOrdersRequest {
            method: "cancel_all".to_owned(),
            params: CancelAllOrdersParams { token },
            req_id: None,
        }
    }
}
