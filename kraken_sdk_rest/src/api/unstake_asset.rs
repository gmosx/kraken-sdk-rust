use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

/// - https://docs.kraken.com/rest/#operation/unstake
/// - https://api.kraken.com/0/private/Unstake
#[must_use = "Does nothing until you send or execute it"]
pub struct UnstakeAssetRequest {
    client: Client,
    /// Asset to unstake (asset ID or altname). Must be a valid staking asset (e.g. XBT.M, XTZ.S, ADA.S)
    asset: String,
    /// Amount of the asset to unstake.
    amount: String,
}

impl UnstakeAssetRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let query = format!("asset={}&amount={}", self.asset, self.amount);

        self.client
            .send_private("/0/private/Unstake", Some(query))
            .await
    }

    pub async fn send(self) -> Result<UnstakeAssetResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct UnstakeAssetResponse {
    pub refid: String,
}

impl Client {
    pub fn unstake_asset(&self, asset: impl Into<String>, amount: &str) -> UnstakeAssetRequest {
        UnstakeAssetRequest {
            client: self.clone(),
            asset: asset.into(),
            amount: amount.to_string(),
        }
    }
}
