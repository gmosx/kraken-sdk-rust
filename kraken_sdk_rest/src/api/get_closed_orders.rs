use crate::{Client, OrderDescription, Result};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

// TODO: This endpoint is under construction. Don't use yet!
// TODO: Rename to `get_close_orders_history`.

/// - <https://docs.kraken.com/rest/#operation/getClosedOrders>
/// - <https://api.kraken.com/0/private/ClosedOrders>
#[must_use = "Does nothing until you send or execute it"]
pub struct GetClosedOrdersRequest {
    client: Client,
    trades: Option<bool>,
    userref: Option<i32>,
    start: Option<i64>,
    end: Option<i64>,
    offset: Option<usize>,
    // TODO:
    // start = starting unix timestamp or order tx id of results (optional.  exclusive)
    // end = ending unix timestamp or order tx id of results (optional.  inclusive)
    // ofs = result offset
    // closetime = which time to use (optional)
    //     open
    //     close
    //     both (default)
}

impl GetClosedOrdersRequest {
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

    pub fn start(self, start: i64) -> Self {
        Self {
            start: Some(start),
            ..self
        }
    }

    pub fn end(self, end: i64) -> Self {
        Self {
            end: Some(end),
            ..self
        }
    }

    pub fn offset(self, offset: usize) -> Self {
        Self {
            offset: Some(offset),
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

        if let Some(start) = self.start {
            query.push(format!("start={}", start));
        }

        if let Some(end) = self.end {
            query.push(format!("end={}", end));
        }

        if let Some(offset) = self.offset {
            query.push(format!("ofs={}", offset));
        }

        let query = if query.is_empty() {
            None
        } else {
            Some(query.join("&"))
        };

        self.client
            .send_private("/0/private/ClosedOrders", query)
            .await
    }

    pub async fn send(self) -> Result<GetClosedOrdersResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct ClosedOrderInfo {
    pub userref: Option<i32>,
    pub status: String,
    pub descr: OrderDescription,
    pub oflags: String,
    pub opentm: f64,
    pub closetm: f64,
    pub expiretm: f64,
    pub vol: String,
    pub vol_exec: String,
    pub fee: String,
    pub misc: String,
    pub limitprice: String,
    pub refid: Option<String>,
    pub reason: Option<String>,
}

// TODO: not fully implemented yet, use JsonValue instead!
#[derive(Debug, Deserialize)]
pub struct GetClosedOrdersResponse {
    pub closed: HashMap<String, ClosedOrderInfo>,
    pub count: usize,
}

impl Client {
    pub fn get_closed_orders(&self) -> GetClosedOrdersRequest {
        GetClosedOrdersRequest {
            client: self.clone(),
            trades: None,
            userref: None,
            start: None,
            end: None,
            offset: None,
        }
    }
}
