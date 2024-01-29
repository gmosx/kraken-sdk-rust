use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

/// - <https://docs.kraken.com/rest/#tag/Funding/operation/getWithdrawalAddresses>
/// - <https://api.kraken.com/0/private/getWithdrawalAddresses>
#[must_use = "Does nothing until you send or execute it"]
pub struct GetWithdrawalAddressesRequest {
    client: Client,
    /// Filter addresses for specific asset
    asset: Option<String>,
    /// Filter addresses for specific asset class
    aclass: Option<String>,
    /// Filter addresses for specific method
    method: Option<String>,
}

impl GetWithdrawalAddressesRequest {
    pub fn asset(self, asset: impl Into<String>) -> Self {
        Self {
            asset: Some(asset.into()),
            ..self
        }
    }

    pub fn aclass(self, aclass: impl Into<String>) -> Self {
        Self {
            aclass: Some(aclass.into()),
            ..self
        }
    }

    pub fn method(self, method: impl Into<String>) -> Self {
        Self {
            method: Some(method.into()),
            ..self
        }
    }

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let mut query: Vec<String> = Vec::new();

        if let Some(asset) = &self.asset {
            query.push(format!("asset={}", asset));
        }

        if let Some(aclass) = &self.aclass {
            query.push(format!("aclass={}", aclass));
        }

        if let Some(method) = &self.method {
            query.push(format!("method={}", method));
        }

        let maybe_query = if query.is_empty() {
            None
        } else {
            Some(query.join("&"))
        };

        self.client
            .send_private("/0/private/WithdrawAddresses", maybe_query)
            .await
    }

    pub async fn send(self) -> Result<GetWithdrawalAddressesResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct WithdrawalAddress {
    pub address: String,
    pub asset: String,
    pub method: String,
    pub key: String,
    pub verified: bool,
}

pub type GetWithdrawalAddressesResponse = Vec<WithdrawalAddress>;

impl Client {
    pub fn get_withdrawal_addresses(&self) -> GetWithdrawalAddressesRequest {
        GetWithdrawalAddressesRequest {
            client: self.clone(),
            asset: None,
            aclass: None,
            method: None,
        }
    }
}
