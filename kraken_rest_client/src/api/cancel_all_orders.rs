use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

/// - <https://docs.kraken.com/rest/#tag/Trading/operation/cancelAllOrders>
/// - <https://api.kraken.com/0/private/CancelAll>
#[must_use = "Does nothing until you send or execute it"]
pub struct CancelAllOrdersRequest {
    client: Client,
}

impl CancelAllOrdersRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        self.client.send_private("/0/private/CancelAll", None).await
    }

    pub async fn send(self) -> Result<CancelAllOrdersResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct CancelAllOrdersResponse {
    pub count: i32,
}

impl Client {
    pub fn cancel_all_orders(&self) -> CancelAllOrdersRequest {
        CancelAllOrdersRequest {
            client: self.clone(),
        }
    }
}
