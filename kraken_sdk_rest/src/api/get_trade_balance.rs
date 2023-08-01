use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

/// - <https://docs.kraken.com/rest/#operation/getTradeBalance>
/// - <https://api.kraken.com/0/private/TradeBalance>
#[must_use = "Does nothing until you send or execute it"]
pub struct GetTradeBalanceRequest {
    client: Client,
    asset: Option<String>,
}

impl GetTradeBalanceRequest {
    /// Base asset used to determine balance (default = ZUSD)
    pub fn asset(self, asset: &str) -> Self {
        Self {
            asset: Some(asset.to_string()),
            ..self
        }
    }

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let query = self.asset.map(|asset| format!("asset={}", asset));

        self.client
            .send_private("/0/private/TradeBalance", query)
            .await
    }

    pub async fn send(self) -> Result<GetTradeBalanceResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct GetTradeBalanceResponse {
    /// Combined balance of all currencies
    #[serde(rename = "eb")]
    pub equivalent_balance: String,
    /// Combined balance of all equity currencies
    #[serde(rename = "tb")]
    pub trade_balance: String,
    /// Margin amount of open positions
    #[serde(rename = "m")]
    pub margin: String,
    /// Unrealized net profit/loss of open positions
    #[serde(rename = "n")]
    pub unrealized_net_pnl: String,
    /// Cost basis of open positions
    #[serde(rename = "c")]
    pub cost_basis: String,
    /// Current floating valuation of open positions
    #[serde(rename = "v")]
    pub valuation: String,
    /// Equity: trade balance + unrealized net profit/loss
    #[serde(rename = "e")]
    pub equity: String,
    /// Free margin: Equity - initial margin (maximum margin available to open new positions)
    #[serde(rename = "mf")]
    pub free_margin: String,
    /// Margin level: (equity / initial margin) * 100
    #[serde(rename = "ml")]
    pub margin_level: Option<String>,
}

impl Client {
    pub fn get_trade_balance(&self) -> GetTradeBalanceRequest {
        GetTradeBalanceRequest {
            client: self.clone(),
            asset: None,
        }
    }
}
