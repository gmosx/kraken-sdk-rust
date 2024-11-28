use crate::{Client, OrderDescription, Result};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

/// - <https://docs.kraken.com/rest/#operation/getOrdersInfo>
/// - <https://api.kraken.com/0/private/QueryOrders>
#[must_use = "Does nothing until you send or execute it"]
pub struct QueryOrdersInfoRequest {
    client: Client,
    /// Whether or not to include trades related to position in output (default = false)
    trades: Option<bool>,
    userref: Option<i32>,
    /// Comma delimited list of transaction IDs to query info about (50 maximum)
    txid: String,
}

impl QueryOrdersInfoRequest {
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
        let mut query: Vec<String> = vec![format!("txid={}", self.txid)];

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
            .send_private("/0/private/QueryOrders", query)
            .await
    }

    pub async fn send(self) -> Result<QueryOrdersInfoResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct OrderInfo {
    pub userref: Option<i32>,
    pub status: String,
    pub descr: OrderDescription,
    pub oflags: String,
    pub opentm: f64,
    pub expiretm: f64,
    pub vol: String,
    pub vol_exec: String,
    pub cost: String,
    pub fee: String,
    pub misc: String,
    pub price: String,
    pub limitprice: String,
    pub refid: Option<String>,
    pub reason: Option<String>,
}

pub type QueryOrdersInfoResponse = HashMap<String, OrderInfo>;

impl Client {
    /// Retrieve information about specific orders.
    ///
    /// ## Example
    ///
    /// let orders = client
    ///     .query_orders_info("OXEHQQ-R25RV-NDOGKM,ORQC1X-TSQHB-KIEE7I")
    ///     .send()
    ///     .await?;
    /// ()
    pub fn query_orders_info(&self, txid: &str) -> QueryOrdersInfoRequest {
        QueryOrdersInfoRequest {
            client: self.clone(),
            trades: None,
            userref: None,
            txid: txid.to_owned(),
        }
    }
}
