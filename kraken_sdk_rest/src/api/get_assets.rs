use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

/// - <https://docs.kraken.com/rest/#operation/getAssetInfo>
/// - <https://api.kraken.com/0/public/Assets>
#[must_use = "Does nothing until you send or execute it"]
pub struct GetAssetsRequest {
    client: Client,
    asset: Option<String>,
}

impl GetAssetsRequest {
    /// Comma delimited list of assets to get info on.
    /// (default = all for given asset class)
    pub fn asset(self, asset: impl Into<String>) -> Self {
        Self {
            asset: Some(asset.into()),
            ..self
        }
    }

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let url = if let Some(asset) = &self.asset {
            format!("/0/public/Assets?asset={}", asset)
        } else {
            String::from("/0/public/Assets")
        };

        self.client.send_public(&url).await
    }

    pub async fn send(self) -> Result<GetAssetsResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct Asset {
    pub altname: String,
    pub aclass: String,
    pub decimals: u32,
    pub display_decimals: u32,
}

pub type GetAssetsResponse = HashMap<String, Asset>;

impl Client {
    pub fn get_assets(&self) -> GetAssetsRequest {
        GetAssetsRequest {
            client: self.clone(),
            asset: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{AssetName, Client};

    #[tokio::test]
    async fn get_assets() {
        let client = Client::default();

        let resp = client.get_assets().asset("DOT,XXRP,XXMR").send().await;

        match resp {
            Ok(resp) => println!("{:?}", resp),
            Err(error) => eprintln!("{:?}", error),
        }

        let asset = AssetName::from("XRP");
        let resp = client.get_assets().asset(&asset).send().await;

        match resp {
            Ok(resp) => println!("{:?}", resp),
            Err(error) => eprintln!("{:?}", error),
        }
    }
}
