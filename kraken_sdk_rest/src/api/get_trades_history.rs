use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;

/// Retrieve information about trades/fills. 50 results are returned at a time,
/// the most recent by default.
/// Unless otherwise stated, costs, fees, prices, and volumes are specified with
/// the precision for the asset pair (pair_decimals and lot_decimals), not the
/// individual assets' precision (decimals).
///
/// - https://docs.kraken.com/rest/#operation/getTradeHistory
/// - https://api.kraken.com/0/private/TradesHistory
#[must_use = "Does nothing until you send or execute it"]
pub struct GetTradesHistoryRequest {
    client: Client,
    // TODO: make this typed.
    trade_type: Option<String>,
    trades: Option<bool>,
    /// starting unix timestamp or order tx id of results
    start: Option<i64>,
    /// end = ending unix timestamp or order tx id of results (optional.  inclusive)
    end: Option<i64>,
    /// result offset
    ofs: Option<i64>,
}

impl GetTradesHistoryRequest {
    pub fn trade_type(self, trades: bool) -> Self {
        Self {
            trades: Some(trades),
            ..self
        }
    }

    /// Whether or not to include trades in output (default = false)
    pub fn trades(self, trades: bool) -> Self {
        Self {
            trades: Some(trades),
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

    pub fn ofs(self, ofs: i64) -> Self {
        Self {
            ofs: Some(ofs),
            ..self
        }
    }

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let mut query: Vec<String> = Vec::new();

        if let Some(trade_type) = self.trade_type {
            query.push(format!("type={}", trade_type));
        }

        if let Some(true) = self.trades {
            query.push(String::from("trades=true"));
        }

        if let Some(start) = self.start {
            query.push(format!("start={}", start));
        }

        if let Some(end) = self.end {
            query.push(format!("end={}", end));
        }

        if let Some(ofs) = self.ofs {
            query.push(format!("ofs={}", ofs));
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

    pub async fn send(self) -> Result<GetTradesHistoryResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TradeInfo {
    pub ordertxid: String,
    pub postxid: Option<String>,
    pub pair: String,
    pub time: f64,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub orderside: String,
    pub ordertype: String,
    pub price: String,
    pub cost: String,
    pub fee: String,
    pub vol: String,
    pub margin: String,
    pub misc: String,
    // TODO: add position related fields.
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTradesHistoryResponse {
    pub trades: HashMap<String, TradeInfo>,
    pub count: i32,
}

impl Client {
    pub fn get_trades_history(&self) -> GetTradesHistoryRequest {
        GetTradesHistoryRequest {
            client: self.clone(),
            trade_type: None,
            trades: None,
            start: None,
            end: None,
            ofs: None,
        }
    }
}
