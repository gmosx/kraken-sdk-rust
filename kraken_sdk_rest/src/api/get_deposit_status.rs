use crate::{Client, OrderDescription, Result};
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;

/// - https://docs.kraken.com/rest/#tag/User-Funding/operation/getDepositStatus
/// - https://api.kraken.com/0/private/DepositStatus
#[must_use = "Does nothing until you send or execute it"]
pub struct GetDepositStatusRequest {
    client: Client,
    asset: String,
    method: Option<String>,
}

impl GetDepositStatusRequest {
    /// Name of the deposit method
    pub fn method(self, method: String) -> Self {
        Self {
            method: Some(method),
            ..self
        }
    }

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let mut query = format!("asset={}", self.asset);

        if let Some(method) = self.method {
            query.push_str(format!("&method={}", method).as_str());
        }

        self.client
            .send_private("/0/private/DepositStatus", Some(query))
            .await
    }

    pub async fn send(self) -> Result<Vec<DepositStatus>> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct DepositStatus {
    pub method: String,
    pub aclass: String,
    pub asset: String,
    pub refid: String,
    pub txid: String,
    pub info: String,
    pub amount: String,
    pub fee: String,
    pub time: u64,
    pub status: String,
    #[serde(rename(deserialize = "status-prop"))]
    pub status_prop: Option<String>,
}

impl Client {
    pub fn get_deposit_status(&self, asset: impl Into<String>) -> GetDepositStatusRequest {
        GetDepositStatusRequest {
            client: self.clone(),
            asset: asset.into(),
            method: None,
        }
    }
}
