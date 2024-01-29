use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

/// - <https://docs.kraken.com/rest/#tag/Funding/operation/getWithdrawalMethods>
/// - <https://api.kraken.com/0/private/WithdrawMethods>
#[must_use = "Does nothing until you send or execute it"]
pub struct GetWithdrawMethodsRequest {
    client: Client,
    /// Asset being withdrawn
    asset: String,
}

impl GetWithdrawMethodsRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let query = format!("asset={}", self.asset);

        self.client
            .send_private("/0/private/WithdrawMethods", Some(query))
            .await
    }

    pub async fn send(self) -> Result<Vec<WithdrawMethods>> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct WithdrawMethods {
    pub asset: String,
    pub method: String,
    pub network: String,
    pub minimum: String,
}

impl Client {
    pub fn get_withdrawal_methods(&self, asset: impl Into<String>) -> GetWithdrawMethodsRequest {
        GetWithdrawMethodsRequest {
            client: self.clone(),
            asset: asset.into(),
        }
    }
}
