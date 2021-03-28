use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

/// - https://www.kraken.com/features/api#get-ticker-info
/// - https://api.kraken.com/0/public/Ticker
#[must_use = "Does nothing until you send or execute it"]
pub struct GetTickersRequestBuilder {
    client: Client,
    pair: Option<String>,
}

impl GetTickersRequestBuilder {
    /// Comma delimited list of asset pairs to get info on.
    pub fn pair(self, pair: &str) -> Self {
        Self {
            pair: Some(String::from(pair)),
            ..self
        }
    }

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let url = if let Some(pair) = &self.pair {
            format!("/0/public/Ticker?pair={}", pair)
        } else {
            String::from("/0/public/Ticker")
        };

        self.client.send_public(&url).await
    }

    pub async fn send(self) -> Result<GetTickersResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct Ticker {
    /// ask array(<price>, <whole lot volume>, <lot volume>),
    pub a: Vec<String>,
    /// bid array(<price>, <whole lot volume>, <lot volume>),
    pub b: Vec<String>,
    /// last trade closed array(<price>, <lot volume>),
    pub c: Vec<String>,
    /// volume array(<today>, <last 24 hours>),
    pub v: Vec<String>,
    /// volume weighted average price array(<today>, <last 24 hours>),
    pub p: Vec<String>,
    /// number of trades array(<today>, <last 24 hours>),
    pub t: Vec<i32>,
    /// low array(<today>, <last 24 hours>),
    pub l: Vec<String>,
    /// high array(<today>, <last 24 hours>),
    pub h: Vec<String>,
    /// today's opening price
    pub o: String,
}

pub type GetTickersResponse = HashMap<String, Ticker>;

impl Client {
    pub fn get_tickers(&self) -> GetTickersRequestBuilder {
        GetTickersRequestBuilder {
            client: self.clone(),
            pair: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[test]
    fn get_tickers() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let client = Client::default();

            let resp = client.get_tickers().pair("XXBTZUSD").send().await;

            match resp {
                Ok(resp) => println!("{:?}", resp),
                Err(error) => eprintln!("{:?}", error),
            }
        });
    }
}
