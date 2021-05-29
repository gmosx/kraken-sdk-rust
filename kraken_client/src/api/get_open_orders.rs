use crate::{Client, OrderInfo, Result};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

/// - https://www.kraken.com/features/api#get-open-orders
/// - https://api.kraken.com/0/private/OpenOrders
#[must_use = "Does nothing until you send or execute it"]
pub struct GetOpenOrdersRequest {
    client: Client,
    trades: Option<bool>,
    userref: Option<i32>,
}

impl GetOpenOrdersRequest {
    /// Whether or not to include trades in output (default = false)
    pub fn trades(self, trades: bool) -> Self {
        Self {
            trades: Some(trades),
            ..self
        }
    }

    /// Restrict results to given user reference id
    pub fn userref(self, userref: i32) -> Self {
        Self {
            userref: Some(userref),
            ..self
        }
    }

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let mut query: Vec<String> = Vec::new();

        if let Some(true) = self.trades {
            query.push(String::from("trades=true"));
        }

        if let Some(userref) = self.userref {
            query.push(format!("userref={}", userref));
        }

        let query = if query.is_empty() {
            None
        } else {
            Some(query.join("&"))
        };

        self.client
            .send_private("/0/private/OpenOrders", query)
            .await
    }

    pub async fn send(self) -> Result<GetOpenOrdersResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct OpenOrderInfo {
    pub status: String,
    pub cost: String,
    pub descr: OrderInfo,
    pub opentm: f64,
    pub oflags: String,
    pub fee: String,
    pub vol: String,
    pub vol_executed: Option<String>,
    pub userref: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct GetOpenOrdersResponse {
    pub open: HashMap<String, OpenOrderInfo>,
}

impl Client {
    pub fn get_open_orders(&self) -> GetOpenOrdersRequest {
        GetOpenOrdersRequest {
            client: self.clone(),
            trades: None,
            userref: None,
        }
    }
}
