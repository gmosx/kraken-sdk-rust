use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

/// - https://www.kraken.com/features/api#get-order-book
/// - https://api.kraken.com/0/public/Depth
#[must_use = "Does nothing until you send or execute it"]
pub struct GetOrderBookRequest {
    client: Client,
    pair: String,
    count: Option<i32>,
}

impl GetOrderBookRequest {
    /// Maximum number of asks/bids
    pub fn count(self, count: i32) -> Self {
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
#[derive(Debug, Deserialize)]
pub struct OrderBookTier(String, String, i32);

#[derive(Debug, Deserialize)]
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

    #[test]
    fn get_order_book() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let client = Client::default();

            let resp = client.get_order_book("XXBTZUSD").count(5).send().await;

            match resp {
                Ok(resp) => println!("{:?}", resp),
                Err(error) => eprintln!("{:?}", error),
            }
        });
    }
}
