use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

// TODO: consider renaming to `get_staking_assets`.

/// - <https://docs.kraken.com/rest/#operation/getStakingAssetInfo>
/// - <https://api.kraken.com/0/private/Staking/Assets>
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
pub struct MinimumAmount {
    pub unstaking: String,
    pub staking: String,
}

#[derive(Debug, Deserialize)]
pub struct StakeableAsset {
    pub asset: String,
    pub staking_asset: String,
    pub method: Option<String>,
    pub on_chain: Option<bool>,
    pub minimum_amount: Option<MinimumAmount>,
    pub enabled_for_user: Option<bool>,
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
