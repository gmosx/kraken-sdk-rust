use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

/// - <https://docs.kraken.com/rest/#tag/Funding/operation/getWithdrawalMethods>
/// - <https://api.kraken.com/0/private/WithdrawMethods>
#[must_use = "Does nothing until you send or execute it"]
pub struct GetWithdrawMethodsRequest {
    client: Client,
    /// Filter methods for specific asset
    asset: Option<String>,
    /// Filter methods for specific asset class
    aclass: Option<String>,
    /// Filter methods for specific network
    network: Option<String>,
}

impl GetWithdrawMethodsRequest {
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

    pub fn network(self, network: impl Into<String>) -> Self {
        Self {
            network: Some(network.into()),
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

        if let Some(network) = &self.network {
            query.push(format!("network={}", network));
        }

        let maybe_query = if query.is_empty() {
            None
        } else {
            Some(query.join("&"))
        };

        self.client
            .send_private("/0/private/WithdrawMethods", maybe_query)
            .await
    }

    pub async fn send(self) -> Result<GetWithdrawMethodsResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct WithdrawMethod {
    pub asset: String,
    pub method: String,
    pub network: String,
    pub minimum: String,
}

pub type GetWithdrawMethodsResponse = Vec<WithdrawMethod>;

impl Client {
    pub fn get_withdrawal_methods(&self) -> GetWithdrawMethodsRequest {
        GetWithdrawMethodsRequest {
            client: self.clone(),
            asset: None,
            aclass: None,
            network: None,
        }
    }
}
