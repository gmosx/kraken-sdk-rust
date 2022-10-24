use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;

/// - https://docs.kraken.com/rest/#tag/User-Data/operation/getLedgers
/// - https://api.kraken.com/0/private/Ledgers
#[must_use = "Does nothing until you send or execute it"]
pub struct GetLedgersRequest {
    client: Client,
    asset: Option<String>,
    aclass: Option<String>,
    ledger_type: Option<String>,
    start: Option<u64>,
    end: Option<u64>,
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
    pub fn ledger_type(self, ledger_type: impl Into<String>) -> Self {
        Self {
            aclass: Some(ledger_type.into()),
            ..self
        }
    }

    /// Starting unix timestamp or ledger ID of results (exclusive)
    pub fn start(self, start: u64) -> Self {
        Self {
            start: Some(start),
            ..self
        }
    }

    /// Ending unix timestamp or ledger ID of results (inclusive)
    pub fn end(self, end: u64) -> Self {
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
            query.push(format!("asset={}", asset));
        }

        if let Some(aclass) = self.aclass {
            query.push(format!("aclass={}", aclass));
        }

        if let Some(ledger_type) = self.ledger_type {
            query.push(format!("type={}", ledger_type));
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum LedgerType {
    Trade,
    Deposit,
    Withdrawal,
    Transfer,
    Margin,
    Rollover,
    Spend,
    Receive,
    Settled,
    Adjustment,
    Staking,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum Subtype {
    SpotFromStaking,
    SpotToStaking,
    StakingFromSpot,
    StakingToSpot,
    SpotFromFutures,
    #[serde(alias = "")]
    None,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum AssetClass {
    Currency,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LedgerEntry {
    /// Reference Id
    pub refid: String,
    /// Unix timestamp of ledger
    pub time: f64,
    /// Type of ledger entry
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub ledger_type: LedgerType,
    /// Additional info relating to the ledger entry type, where applicable
    pub subtype: Subtype,
    /// Asset class
    pub aclass: AssetClass,
    /// Asset
    pub asset: String,
    /// Transaction amount
    pub amount: String,
    /// Transaction fee
    pub fee: String,
    /// Resulting balance
    pub balance: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetLedgersResponse {
    pub ledger: HashMap<String, LedgerEntry>,
    pub count: u32,
}

impl Client {
    pub fn get_ledgers(&self) -> GetLedgersRequest {
        GetLedgersRequest {
            client: self.clone(),
            asset: None,
            aclass: None,
            ledger_type: None,
            start: None,
            end: None,
            ofs: None,
        }
    }
}
