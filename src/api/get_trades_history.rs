use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

// type = type of trade (optional)
//     all = all types (default)
//     any position = any position (open or closed)
//     closed position = positions that have been closed
//     closing position = any trade closing all or part of a position
//     no position = non-positional trades
// trades = whether or not to include trades related to position in output (optional.  default = false)
// start = starting unix timestamp or trade tx id of results (optional.  exclusive)
// end = ending unix timestamp or trade tx id of results (optional.  inclusive)
// ofs = result offset

/// - https://www.kraken.com/features/api#get-trades-history
/// - https://api.kraken.com/0/private/TradesHistory
#[must_use = "Does nothing until you send or execute it"]
pub struct GetTradesHistoryRequestBuilder {
    client: Client,
    trades: Option<bool>,
    start: Option<i32>,
    end: Option<i32>,
    // TODO:
    // start = starting unix timestamp or order tx id of results (optional.  exclusive)
    // end = ending unix timestamp or order tx id of results (optional.  inclusive)
    // ofs = result offset
    // closetime = which time to use (optional)
    //     open
    //     close
    //     both (default)
}

impl GetTradesHistoryRequestBuilder {
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
            .send_private("/0/private/TradesHistory", query)
            .await
    }

    pub async fn send(self) -> Result<GetOpenOrdersResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct ClosedOrderInfo {
    pub status: String,
    pub descr: OrderInfo,
    pub oflags: String,
    pub closetm: f64,
    pub reason: Option<String>,
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

// TODO: not fully implemented yet, use JsonValue instead!
#[derive(Debug, Deserialize)]
pub struct GetOpenOrdersResponse {
    pub closed: HashMap<String, ClosedOrderInfo>,
    pub count: i32,
}

impl Client {
    pub fn get_trades_history(&self) -> GetTradesHistoryRequestBuilder {
        GetTradesHistoryRequestBuilder {
            client: self.clone(),
            trades: None,
            userref: None,
        }
    }
}
