use crate::{Client, JsonValue, Result};
use serde::de::DeserializeOwned;

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

/// - https://www.kraken.com/features/api#get-ohlc-data
/// - https://api.kraken.com/0/public/OHLC
#[must_use = "Does nothing until you send or execute it"]
pub struct GetOhlcDataRequestBuilder {
    client: Client,
    pair: String,
    interval: Option<Interval>,
}

impl GetOhlcDataRequestBuilder {
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
        self.execute().await
    }
}

// TODO: temp solution.
pub type GetOhlcDataResponse = JsonValue;

impl Client {
    pub fn get_ohlc_data(&self, pair: impl Into<String>) -> GetOhlcDataRequestBuilder {
        GetOhlcDataRequestBuilder {
            client: self.clone(),
            pair: pair.into(),
            interval: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, Interval, PairName};

    #[test]
    fn get_ohlc_data() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let client = Client::default();

            let pair = PairName::from("XBT", "USD");
            let resp = client
                .get_ohlc_data(&pair)
                .interval(Interval::Day1)
                .send()
                .await;

            match resp {
                Ok(resp) => println!("{:?}", resp),
                Err(error) => eprintln!("{:?}", error),
            }
        });
    }
}
