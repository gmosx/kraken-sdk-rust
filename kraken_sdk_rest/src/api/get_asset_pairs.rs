use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

/// - https://www.kraken.com/features/api#get-tradable-pairs
/// - https://api.kraken.com/0/public/AssetPairs
#[must_use = "Does nothing until you send or execute it"]
pub struct GetAssetPairsRequest {
    client: Client,
    pair: Option<String>,
}

impl GetAssetPairsRequest {
    /// Comma delimited list of asset pairs to get info on.
    pub fn pair(self, pair: impl Into<String>) -> Self {
        Self {
            pair: Some(pair.into()),
            ..self
        }
    }

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let url = if let Some(pair) = &self.pair {
            format!("/0/public/AssetPairs?pair={}", pair)
        } else {
            String::from("/0/public/AssetPairs")
        };

        self.client.send_public(&url).await
    }

    pub async fn send(self) -> Result<GetAssetPairsResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct FeeSchedule(f64, f64);

#[derive(Debug, Deserialize)]
pub struct PairInfo {
    /// alternate pair name
    pub altname: String,
    /// WebSocket pair name (if available)
    pub wsname: Option<String>,
    /// asset class of base component
    pub aclass_base: String,
    /// asset id of base component
    pub base: String,
    /// asset class of quote component
    pub aclass_quote: String,
    /// asset id of quote component
    pub quote: String,
    /// volume lot size
    pub lot: String,
    /// scaling decimal places for pair
    pub pair_decimals: i32,
    /// scaling decimal places for volume
    pub lot_decimals: i32,
    /// amount to multiply lot volume by to get currency volume
    pub lot_multiplier: i32,
    /// array of leverage amounts available when buying
    pub leverage_buy: Vec<f64>,
    /// array of leverage amounts available when selling
    pub leverage_sell: Vec<f64>,
    /// fee schedule array
    pub fees: Vec<FeeSchedule>,
    /// maker fee schedule array in [volume, percent fee] tuples (if on maker/taker)
    pub fees_maker: Option<Vec<FeeSchedule>>,
    /// volume discount currency
    pub fee_volume_currency: String,
    /// margin call level
    pub margin_call: f64,
    /// stop-out/liquidation margin level
    pub margin_stop: f64,
    /// minimum order volume for pair
    pub ordermin: Option<String>,
}

pub type GetAssetPairsResponse = HashMap<String, PairInfo>;

impl Client {
    pub fn get_asset_pairs(&self) -> GetAssetPairsRequest {
        GetAssetPairsRequest {
            client: self.clone(),
            pair: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, JsonValue, PairName, Result};

    #[tokio::test]
    async fn get_asset_pairs() {
        let client = Client::default();

        let resp = client.get_asset_pairs().pair("XXBTZUSD").send().await;

        match resp {
            Ok(resp) => println!("{:?}", resp),
            Err(error) => eprintln!("{:?}", error),
        }

        let pair = PairName::from("BTC", "USD");
        let resp: Result<JsonValue> = client.get_asset_pairs().pair(&pair).execute().await;

        match resp {
            Ok(resp) => println!("{:?}", resp),
            Err(error) => eprintln!("{:?}", error),
        }
    }
}
