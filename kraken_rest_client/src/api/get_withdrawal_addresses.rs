use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

/// - <https://docs.kraken.com/rest/#tag/Funding/operation/getWithdrawalAddresses>
/// - <https://api.kraken.com/0/private/getWithdrawalAddresses>
#[must_use = "Does nothing until you send or execute it"]
pub struct GetWithdrawalAddressesRequest {
    client: Client,
    /// Asset being withdrawn
    asset: String,
    /// Name of the withdrawal method
    method: String,
}

impl GetWithdrawalAddressesRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let query = format!("asset={}&method={}", self.asset, self.method);
        self.client
            .send_private("/0/private/WithdrawAddresses", Some(query))
            .await
    }

    pub async fn send(self) -> Result<Vec<WithdrawalAddress>> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct WithdrawalAddress {
    pub address: String,
    pub asset: String,
    pub method: String,
    pub key: String,
    pub verified: bool,
}

impl Client {
    pub fn get_withdrawal_addresses(
        &self,
        asset: impl Into<String>,
        method: impl Into<String>,
    ) -> GetWithdrawalAddressesRequest {
        GetWithdrawalAddressesRequest {
            client: self.clone(),
            asset: asset.into(),
            method: method.into(),
        }
    }
}
