use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

/// - https://docs.kraken.com/rest/#operation/getStakingAssetInfo
/// - https://api.kraken.com/0/private/Staking/Assets
#[must_use = "Does nothing until you send or execute it"]
pub struct GetStakeableAssetsRequest {
    client: Client,
}

impl GetStakeableAssetsRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        self.client
            .send_private("/0/private/Staking/Assets", None)
            .await
    }

    pub async fn send(self) -> Result<GetStakeableAssetsResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct StakeableAsset {
    pub asset: String,
    pub staking_asset: String,
    pub method: Option<String>,
    pub on_change: Option<bool>,
    // TODO...
}

pub type GetStakeableAssetsResponse = Vec<StakeableAsset>;

impl Client {
    pub fn get_stakeable_assets(&self) -> GetStakeableAssetsRequest {
        GetStakeableAssetsRequest {
            client: self.clone(),
        }
    }
}
