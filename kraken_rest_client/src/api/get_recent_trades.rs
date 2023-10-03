use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

/// - <https://docs.kraken.com/rest/#tag/Market-Data/operation/getRecentTrades>
/// - <https://api.kraken.com/0/public/Trades>
#[must_use = "Does nothing until you send or execute it"]
pub struct GetRecentTradesRequest {
    client: Client,
    pair: String,
    count: Option<u16>,
    since: Option<String>,
}

impl GetRecentTradesRequest {
    pub fn count(self, count: u16) -> Self {
        Self {
            count: Some(count),
            ..self
        }
    }

    pub fn since(self, since: String) -> Self {
        Self {
            since: Some(since),
            ..self
        }
    }

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let mut url = format!("/0/public/Trades?pair={}", self.pair);

        if let Some(count) = self.count {
            url.push_str(&format!("&count={count}"))
        }

        if let Some(since) = self.since {
            url.push_str(&format!("&since={since}"))
        }

        self.client.send_public(&url).await
    }

    pub async fn send(self) -> Result<GetRecentTradesResponse> {
        self.execute::<GetRecentTradesResponse>().await
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Trade(
    /// price (0)
    pub String,
    /// volume (1)
    pub String,
    /// time (2)
    pub f64,
    /// buy/sell (3)
    pub String,
    /// market/limit (4)
    pub String,
    /// miscellaneous (5)
    pub String,
    /// trade_id (6)
    pub u64,
);

impl Trade {
    pub fn price(&self) -> &str {
        &self.0
    }

    pub fn volume(&self) -> &str {
        &self.1
    }

    pub fn time(&self) -> f64 {
        self.2
    }

    pub fn buy_sell(&self) -> &str {
        &self.3
    }

    pub fn market_limit(&self) -> &str {
        &self.4
    }

    pub fn miscellaneous(&self) -> &str {
        &self.5
    }

    pub fn trade_id(&self) -> u64 {
        self.6
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct GetRecentTradesResponse {
    pub last: String,

    #[serde(flatten)]
    pub pair: HashMap<String, Vec<Trade>>,
}

impl Client {
    pub fn get_recent_trades(&self, pair: impl Into<String>) -> GetRecentTradesRequest {
        GetRecentTradesRequest {
            client: self.clone(),
            pair: pair.into(),
            count: None,
            since: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn get_recent_trades() {
        let client = Client::default();

        let trades = client
            .get_recent_trades("BTC/USD")
            .count(5)
            .since("0".into())
            .send()
            .await;

        if let Ok(trades) = trades {
            assert!(trades.pair["BTC/USD"].len() == 5);
        }
    }
}
