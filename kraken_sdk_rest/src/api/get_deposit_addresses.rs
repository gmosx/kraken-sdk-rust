use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

/// - https://docs.kraken.com/rest/#tag/User-Funding/operation/getDepositAddresses
/// - https://api.kraken.com/0/private/DepositAddresses
#[must_use = "Does nothing until you send or execute it"]
pub struct GetDepositAddressesRequest {
    client: Client,
    /// Asset being deposited
    asset: String,
    /// Name of the deposit method
    method: String,
}

impl GetDepositAddressesRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let mut query = format!("asset={}", self.asset);

        query.push_str(format!("&method={}", self.method).as_str());

        self.client
            .send_private("/0/private/DepositAddresses", Some(query))
            .await
    }

    pub async fn send(self) -> Result<Vec<DepositAddresses>> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct DepositAddresses {
    pub address: String,
    pub expiretm: String,
    pub new: Option<bool>,
}

impl Client {
    pub fn get_deposit_addresses(
        &self,
        asset: impl Into<String>,
        method: impl Into<String>,
    ) -> GetDepositAddressesRequest {
        GetDepositAddressesRequest {
            client: self.clone(),
            asset: asset.into(),
            method: method.into(),
        }
    }
}
