use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

/// - https://www.kraken.com/features/api#cancel-open-order
/// - https://api.kraken.com/0/private/CancelOrder
#[must_use = "Does nothing until you send or execute it"]
pub struct CancelOrderRequestBuilder {
    client: Client,
    /// An order id or a 'userref' id.
    txid: String,
}

impl CancelOrderRequestBuilder {
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
    count: isize,
    pending: Option<bool>,
}

impl Client {
    pub fn cancel_order(&self, txid: &str) -> CancelOrderRequestBuilder {
        CancelOrderRequestBuilder {
            client: self.clone(),
            txid: txid.to_string(),
        }
    }
}
