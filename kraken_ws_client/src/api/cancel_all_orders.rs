//! <https://docs.kraken.com/websockets-v2/#cancel-all-orders>

use crate::client::{PrivateParams, PrivateRequest};

/// Cancels all pending orders.
///
/// <https://docs.kraken.com/websockets-v2/#cancel-all-orders>
pub type CancelAllOrdersRequest = PrivateRequest<()>;

impl CancelAllOrdersRequest {
    pub fn new() -> CancelAllOrdersRequest {
        CancelAllOrdersRequest {
            method: "cancel_all".to_owned(),
            params: PrivateParams::new(()),
            req_id: None,
        }
    }
}
