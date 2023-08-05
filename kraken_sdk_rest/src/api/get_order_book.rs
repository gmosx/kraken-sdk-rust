use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;

/// - <https://docs.kraken.com/rest/#operation/getOrderBook>
/// - <https://api.kraken.com/0/public/Depth>
#[must_use = "Does nothing until you send or execute it"]
pub struct GetOrderBookRequest {
    client: Client,
    pair: String,
    /// Maximum number of asks/bids [1..500], default = 100
    count: Option<u32>,
}

impl GetOrderBookRequest {
    /// Maximum number of asks/bids
    pub fn count(self, count: u32) -> Self {
        Self {
            count: Some(count),
            ..self
        }
    }

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let mut url = format!("/0/public/Depth?pair={}", &self.pair);

        if let Some(count) = &self.count {
            url.push_str(&format!("&count={}", count));
        }

        self.client.send_public(&url).await
    }

    pub async fn send(self) -> Result<GetOrderBookResponse> {
        self.execute().await
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBookTier(pub String, pub String, pub i32);

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBook {
    /// ask side array of array entries(<price>, <volume>, <timestamp>)
    pub asks: Vec<OrderBookTier>,
    /// bid side array of array entries(<price>, <volume>, <timestamp>)
    pub bids: Vec<OrderBookTier>,
}

pub type GetOrderBookResponse = HashMap<String, OrderBook>;

impl Client {
    pub fn get_order_book(&self, pair: &str) -> GetOrderBookRequest {
        GetOrderBookRequest {
            client: self.clone(),
            pair: String::from(pair),
            count: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn get_order_book() {
        let client = Client::default();

        let resp = client.get_order_book("XXBTZUSD").count(5).send().await;

        match resp {
            Ok(resp) => println!("{:?}", resp),
            Err(error) => eprintln!("{:?}", error),
        }
    }
}
