use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

/// - https://docs.kraken.com/rest/#operation/stake
/// - https://api.kraken.com/0/private/stake
#[must_use = "Does nothing until you send or execute it"]
pub struct StakeAssetRequest {
    client: Client,
    /// Asset to stake (asset ID or altname)
    asset: String,
    /// Amount of the asset to stake.
    amount: String,
    /// Name of the staking option to use (refer to the Staking Assets endpoint for the correct method names for each asset)
    method: String, // TODO: make method optional and try to autofill it!
}

impl StakeAssetRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let query = format!(
            "asset={}&amount={}&method={}",
            self.asset, self.amount, self.method
        );

        self.client
            .send_private("/0/private/Stake", Some(query))
            .await
    }

    pub async fn send(self) -> Result<StakeAssetResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct StakeAssetResponse {
    refid: String,
}

impl Client {
    pub fn stake_asset(&self, asset: &str, amount: &str, method: &str) -> StakeAssetRequest {
        StakeAssetRequest {
            client: self.clone(),
            asset: asset.to_string(),
            amount: amount.to_string(),
            method: method.to_string(),
        }
    }
}
