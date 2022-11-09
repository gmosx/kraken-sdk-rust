use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

/// - https://docs.kraken.com/rest/#tag/User-Data/operation/getOpenPositions
/// - https://api.kraken.com/0/private/OpenPositions
#[must_use = "Does nothing until you send or execute it"]
pub struct GetOpenPositionsRequest {
    client: Client,
    docalcs: bool,
}

impl GetOpenPositionsRequest {
    /// Whether to include P&L calculations (default = false)
    pub fn docalcs(self, docalcs: bool) -> Self {
        Self {
            docalcs,
            ..self
        }
    }

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let mut query: Vec<String> = Vec::new();

        if self.docalcs {
            query.push(String::from("docalcs=true"));
        }

        let query = if query.is_empty() {
            None
        } else {
            Some(query.join("&"))
        };

        self.client
            .send_private("/0/private/OpenPositions", query)
            .await
    }

    pub async fn send(self) -> Result<GetOpenPositionsResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct OpenPositionInfo {
    pub ordertxid: String,
    pub posstatus: String,
    pub pair: String,
    pub time: f64,
    pub r#type: String,
    pub ordertype: String,
    pub cost: String,
    pub fee: String,
    pub vol: String,
    pub vol_closed: String,
    pub margin: String,
    pub value: Option<String>,
    pub net: Option<String>,
    pub terms: String,
    pub rollovertm: String,
    pub misc: String,
    pub oflags: String,
}

#[derive(Debug, Deserialize)]
pub struct GetOpenPositionsResponse {
    #[serde(flatten)]
    pub result: HashMap<String, OpenPositionInfo>,
}

impl Client {
    pub fn get_open_positions(&self) -> GetOpenPositionsRequest {
        GetOpenPositionsRequest {
            client: self.clone(),
            docalcs: false,
        }
    }
}
