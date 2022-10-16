use crate::{Client, OrderDescription, Result};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

/// - https://docs.kraken.com/rest/#tag/User-Data/operation/getLedgers
/// - https://api.kraken.com/0/private/Ledgers
#[must_use = "Does nothing until you send or execute it"]
pub struct GetLedgersRequest {
    client: Client,
    asset: Option<String>,
    aclass: Option<String>,
    ledgertype: Option<String>,
    start: Option<u32>,
    end: Option<u32>,
    ofs: Option<u32>,
}

impl GetLedgersRequest {
    /// Comma delimited list of assets to get info on.
    /// (default = all for given asset class)
    pub fn asset(self, asset: impl Into<String>) -> Self {
        Self {
            asset: Some(asset.into()),
            ..self
        }
    }
    /// Asset class
    pub fn aclass(self, aclass: impl Into<String>) -> Self {
        Self {
            aclass: Some(aclass.into()),
            ..self
        }
    }
    /// Type of ledger to retrieve
    pub fn ledgertype(self, ledgertype: impl Into<String>) -> Self {
        Self {
            aclass: Some(ledgertype.into()),
            ..self
        }
    }
    /// Starting unix timestamp or ledger ID of results (exclusive)
    pub fn start(self, start: u32) -> Self {
        Self {
            start: Some(start),
            ..self
        }
    }
    /// Ending unix timestamp or ledger ID of results (inclusive)
    pub fn end(self, end: u32) -> Self {
        Self {
            end: Some(end),
            ..self
        }
    }
    /// Result offset for pagination
    pub fn ofs(self, ofs: u32) -> Self {
        Self {
            ofs: Some(ofs),
            ..self
        }
    }

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let mut query: Vec<String> = Vec::new();

        if let Some(asset) = self.asset {
            query.push(String::from("asset={}"));
        }

        if let Some(aclass) = self.aclass {
            query.push(format!("aclass={}", aclass));
        }

        if let Some(ledgertype) = self.ledgertype {
            query.push(format!("type={}", ledgertype));
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

        self.client.send_private("/0/private/Ledgers", query).await
    }

    pub async fn send(self) -> Result<GetLedgersResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct LedgerEntry {
    pub refid: String,
    pub time: f64,
    #[serde(rename(deserialize = "type"))]
    pub ledger_type: String,
    pub subtype: String,
    pub aclass: String,
    pub asset: String,
    pub amount: String,
    pub fee: String,
    pub balance: String,
}

#[derive(Debug, Deserialize)]
pub struct GetLedgersResponse {
    pub ledger: HashMap<String, LedgerEntry>,
}

impl Client {
    pub fn get_ledgers(&self) -> GetLedgersRequest {
        GetLedgersRequest {
            client: self.clone(),
            asset: None,
            aclass: None,
            ledgertype: None,
            start: None,
            end: None,
            ofs: None,
        }
    }
}
