use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

/// - <https://docs.kraken.com/rest/#tag/User-Funding/operation/getStatusRecentDeposits>
/// - <https://api.kraken.com/0/private/DepositStatus>
#[must_use = "Does nothing until you send or execute it"]
pub struct GetDepositStatusRequest {
    client: Client,
    /// Asset being deposited
    asset: String,
    /// Name of the deposit method
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
    /// Name of deposit method
    pub method: String,
    /// Asset class
    pub aclass: String,
    /// Asset
    pub asset: String,
    /// Reference ID
    pub refid: String,
    /// Transaction ID
    pub txid: String,
    /// Transaction information
    pub info: String,
    /// Amount deposited
    pub amount: String,
    /// Fee paid (not present when the deposit is pending)
    pub fee: Option<String>,
    /// Unix timestamp when request was made
    pub time: u64,
    /// Status of deposit
    pub status: String,
    #[serde(rename = "status-prop")]
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
