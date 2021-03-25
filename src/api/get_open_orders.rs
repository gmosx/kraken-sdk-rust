use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

/// - https://www.kraken.com/features/api#get-open-orders
/// - https://api.kraken.com/0/private/OpenOrders
#[must_use = "Does nothing until you send or execute it"]
pub struct GetOpenOrdersRequestBuilder {
    client: Client,
}

impl GetOpenOrdersRequestBuilder {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        self.client
            .send_private("/0/private/OpenOrders", None)
            .await
    }

    pub async fn send(self) -> Result<GetOpenOrdersResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct OpenOrderInfo {
    pub status: String,
    pub descr: OrderInfo,
    pub oflags: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderInfo {
    // pub ordertxid: Option<String>,
    // pub postxid: Option<String>,
    pub pair: String,
    // pub time: f64,
    #[serde(rename(deserialize = "type"))]
    pub marketside: String,
    pub ordertype: String,
    pub price: String,
    pub price2: String,
    pub leverage: String,
    pub order: String,
    pub close: String,
    // pub cost: String,
    // pub fee: String,
    // pub vol: String,
    // pub margin: String,
    // pub misc: String,
}

#[derive(Debug, Deserialize)]
pub struct GetOpenOrdersResponse {
    pub open: HashMap<String, OpenOrderInfo>,
}

impl Client {
    pub fn get_open_orders(&self) -> GetOpenOrdersRequestBuilder {
        GetOpenOrdersRequestBuilder {
            client: self.clone(),
        }
    }
}
