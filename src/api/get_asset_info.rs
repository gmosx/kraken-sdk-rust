use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

/// - https://www.kraken.com/features/api#get-asset-info
/// - https://api.kraken.com/0/public/Assets
#[must_use = "Does nothing until you send or execute it"]
pub struct GetAssetInfoRequestBuilder {
    client: Client,
    asset: Option<String>,
}

impl GetAssetInfoRequestBuilder {
    /// Comma delimited list of assets to get info on.
    /// (default = all for given asset class)
    pub fn asset(self, asset: &str) -> Self {
        Self {
            asset: Some(String::from(asset)),
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

    pub async fn send(self) -> Result<GetAssetInfoResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct AssetInfo {
    pub altname: String,
    pub aclass: String,
    pub decimals: i32,
    pub display_decimals: i32,
}

pub type GetAssetInfoResponse = HashMap<String, AssetInfo>;

impl Client {
    pub fn get_asset_info(&self) -> GetAssetInfoRequestBuilder {
        GetAssetInfoRequestBuilder {
            client: self.clone(),
            asset: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[test]
    fn get_asset_info() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let client = Client::default();

            let resp = client.get_asset_info().asset("DOT,XXRP,XXMR").send().await;

            match resp {
                Ok(resp) => println!("{:?}", resp),
                Err(error) => eprintln!("{:?}", error),
            }
        });
    }
}
