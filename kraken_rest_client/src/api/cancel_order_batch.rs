use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

/// - <https://docs.kraken.com/rest/#tag/Trading/operation/cancelOrderBatch>
/// - <https://api.kraken.com/0/private/CancelOrderBatch>
#[must_use = "Does nothing until you send or execute it"]
pub struct CancelOrderBatchRequest {
    client: Client,
    /// Open order transaction IDs (txid) or user references (userref), up to a
    /// maximum of 50 total unique IDs/references.
    txid: Vec<String>,
}

impl CancelOrderBatchRequest {
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
}

impl Client {
    pub fn cancel_order(&self, txid: impl Into<Vec<String>>) -> CancelOrderBatchRequest {
        CancelOrderBatchRequest {
            client: self.clone(),
            txid: txid.into(),
        }
    }
}
