use crate::client::{PrivateParams, PrivateRequest, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CancelAllOrdersAfterParams {
    /// Duration (in seconds) to set/extend the timer by. Note: should be less than 86400 seconds.
    pub timeout: i32,
}

/// `cancel_all_orders_after` provides a "Dead Man's Switch" mechanism to protect
/// the client from network malfunction, extreme latency or unexpected matching
/// engine downtime. The client can send a request with a timeout (in seconds),
/// that will start a countdown timer which will cancel all client orders when
/// the timer expires. The client has to keep sending new requests to push back
/// the trigger time, or deactivate the mechanism by specifying a timeout of 0.
/// If the timer expires, all orders are cancelled and then the timer remains
/// disabled until the client provides a new (non-zero) timeout.
///
/// <https://docs.kraken.com/websockets-v2/#cancel-all-orders-after>
pub type CancelAllOrdersAfterRequest = PrivateRequest<CancelAllOrdersAfterParams>;

impl CancelAllOrdersAfterRequest {
    pub fn new(timeout: i32) -> CancelAllOrdersAfterRequest {
        CancelAllOrdersAfterRequest {
            method: "cancel_all_orders_after".to_owned(),
            params: PrivateParams::new(CancelAllOrdersAfterParams { timeout }),
            req_id: None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersAfterResult {
    pub current_time: String,
    pub trigger_time: String,
}

pub type CancelAllOrdersAfterResponse = Response<CancelAllOrdersAfterResult>;
