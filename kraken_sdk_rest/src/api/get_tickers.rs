use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

/// - https://docs.kraken.com/rest/#operation/getTickerInformation
/// - https://api.kraken.com/0/public/Ticker
#[must_use = "Does nothing until you send or execute it"]
pub struct GetTickersRequest {
    client: Client,
    /// Comma delimited list of asset pairs to get info on.
    pair: String,
}

impl GetTickersRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let url = format!("/0/public/Ticker?pair={}", self.pair);

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
    pub fn get_tickers(&self, pair: &str) -> GetTickersRequest {
        GetTickersRequest {
            client: self.clone(),
            pair: pair.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn get_tickers() {
        let client = Client::default();

        let resp = client.get_tickers("XXBTZUSD,DOTUSD").send().await;

        match resp {
            Ok(resp) => println!("{:?}", resp),
            Err(error) => eprintln!("{:?}", error),
        }
    }
}
