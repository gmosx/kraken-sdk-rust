use crate::{error::Error, Client, JsonValue, Result};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

/// Time frame interval.
pub enum Interval {
    Min1 = 1,
    Min5 = 5,
    Min15 = 15,
    Min30 = 30,
    Hour1 = 60,
    Hour4 = 240,
    Day1 = 1_440,
    Day7 = 10_080,
    Day15 = 21_600,
}

/// - https://docs.kraken.com/rest/#operation/getOHLCData
/// - https://api.kraken.com/0/public/OHLC
#[must_use = "Does nothing until you send or execute it"]
pub struct GetOhlcDataRequest {
    client: Client,
    pair: String,
    interval: Option<Interval>,
}

impl GetOhlcDataRequest {
    pub fn interval(self, interval: Interval) -> Self {
        Self {
            interval: Some(interval),
            ..self
        }
    }

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let mut url = format!("/0/public/OHLC?pair={}", self.pair);

        if let Some(interval) = self.interval {
            url.push_str(&format!("&interval={}", interval as u32))
        }

        self.client.send_public(&url).await
    }

    pub async fn send(self) -> Result<GetOhlcDataResponse> {
        // TODO: how tro avoid this?
        let pair = self.pair.clone();

        let resp = self.execute::<GetOhlcDataRawResponse>().await?;

        if let Some(value) = resp.get(&pair) {
            if let Ok(ohlc_data) = serde_json::from_value(value.clone()) {
                Ok(ohlc_data)
            } else {
                Err(Error::internal("cannot deserialize OHLC data"))
            }
        } else {
            Err(Error::internal("no OHLC data"))
        }
    }
}

// TODO: better name?
#[derive(Debug, Deserialize, Clone)]
pub struct OHLC(
    /// time (0)
    pub i64,
    /// open (1)
    pub String,
    /// high (2)
    pub String,
    /// low (3)
    pub String,
    /// close (4)
    pub String,
    /// vwap (5)
    pub String,
    /// volume (6)
    pub String,
    /// count (7)
    pub u64,
);

impl OHLC {
    pub fn time(&self) -> i64 {
        self.0
    }

    pub fn open(&self) -> &String {
        &self.1
    }

    pub fn high(&self) -> &String {
        &self.2
    }

    pub fn low(&self) -> &String {
        &self.3
    }

    pub fn close(&self) -> &String {
        &self.4
    }

    pub fn vwap(&self) -> &String {
        &self.5
    }

    pub fn volume(&self) -> &String {
        &self.6
    }

    pub fn count(&self) -> u64 {
        self.7
    }
}

pub type GetOhlcDataRawResponse = HashMap<String, JsonValue>;
pub type GetOhlcDataResponse = Vec<OHLC>;

impl Client {
    pub fn get_ohlc_data(&self, pair: impl Into<String>) -> GetOhlcDataRequest {
        GetOhlcDataRequest {
            client: self.clone(),
            pair: pair.into(),
            interval: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, Interval, PairName};

    #[tokio::test]
    async fn get_ohlc_data() {
        let client = Client::default();

        let pair = PairName::from("XBT", "USD");
        let ohlc_bars = client
            .get_ohlc_data(&pair)
            .interval(Interval::Day1)
            .send()
            .await;

        // dbg!(&ohlc_bars);

        if let Ok(ohlc_bars) = ohlc_bars {
            assert!(!ohlc_bars.is_empty());
        }
    }
}
