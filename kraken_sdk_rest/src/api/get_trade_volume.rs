use std::collections::HashMap;

use crate::{Client, Result};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// - <https://docs.kraken.com/rest/#tag/User-Data/operation/getTradeVolume>
/// - <https://api.kraken.com/0/private/TradeVolume>
#[must_use = "Does nothing until you send or execute it"]
pub struct GetTradeVolumeRequest {
    client: Client,
    pair: Option<String>,
    fee_info: Option<bool>
}

impl GetTradeVolumeRequest {
    /// Whether or not to include the fee information in the output
    pub fn fee_info(self, fee_info: bool) -> Self {
        Self {
            fee_info: Some(fee_info),
            ..self
        }
    }

    /// Restrict results to a given trading pair
    pub fn pair(self, pair: String) -> Self {
        Self {
            pair: Some(pair),
            ..self
        }
    }

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let mut query: Vec<String> = Vec::new();

        if let Some(pair) = self.pair {
            query.push(format!("pair={}", pair));
        }

        if let Some(true) = self.fee_info {
            query.push(String::from("fee-info=true"));
        }

        let query = if query.is_empty() {
            None
        } else {
            Some(query.join("&"))
        };

        self.client.send_private("/0/private/TradeVolume", query).await
    }

    pub async fn send(self) -> Result<GetTradeVolumeResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FeeTierInfo {
    fee: String,
    #[serde(rename = "minfee")]
    min_fee: String,
    #[serde(rename = "maxfee")]
    max_fee: String,
    #[serde(rename = "nextfee")]
    next_fee: Option<String>,
    #[serde(rename = "tiervolume")]
    tier_volume: Option<String>,
    #[serde(rename = "nextvolume")]
    next_volume: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTradeVolumeResponse {
    pub currency: String,
    pub volume: String,
    pub fees: HashMap<String, FeeTierInfo>,
    pub fees_maker: HashMap<String, FeeTierInfo>
}

impl Client {
     pub fn get_trade_volume(&self) -> GetTradeVolumeRequest {
        GetTradeVolumeRequest {
            client: self.clone(),
            pair: None,
            fee_info: None
        }
    }
}
