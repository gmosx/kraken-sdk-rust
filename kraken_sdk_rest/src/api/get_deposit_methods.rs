use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

/// - https://docs.kraken.com/rest/#tag/User-Funding/operation/getDepositMethods
/// - https://api.kraken.com/0/private/DepositMethods
#[must_use = "Does nothing until you send or execute it"]
pub struct GetDepositMethodsRequest {
    client: Client,
    /// Asset being deposited
    asset: String,
}

impl GetDepositMethodsRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let query = format!("asset={}", self.asset);

        self.client
            .send_private("/0/private/DepositMethods", Some(query))
            .await
    }

    pub async fn send(self) -> Result<Vec<DepositMethods>> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct DepositMethods {
    pub method: String,
    pub fee: Option<String>,
    #[serde(rename = "address-setup-fee")]
    pub address_setup_fee: Option<String>,
    #[serde(rename = "gen-address")]
    pub gen_address: Option<bool>,
}

impl Client {
    pub fn get_deposit_methods(&self, asset: impl Into<String>) -> GetDepositMethodsRequest {
        GetDepositMethodsRequest {
            client: self.clone(),
            asset: asset.into(),
        }
    }
}
