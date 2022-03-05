use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

/// - https://docs.kraken.com/rest/#operation/cancelOrder
/// - https://api.kraken.com/0/private/CancelOrder
#[must_use = "Does nothing until you send or execute it"]
pub struct CancelOrderRequest {
    client: Client,
    /// An order id or a 'userref' id.
    txid: String,
}

impl CancelOrderRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let query = format!("txid={}", self.txid);

        self.client
            .send_private("/0/private/CancelOrder", Some(query))
            .await
    }

    pub async fn send(self) -> Result<CancelOrderResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct CancelOrderResponse {
    pub count: i32,
    pub pending: Option<bool>,
}

impl Client {
    pub fn cancel_order(&self, txid: &str) -> CancelOrderRequest {
        CancelOrderRequest {
            client: self.clone(),
            txid: txid.to_string(),
        }
    }
}
