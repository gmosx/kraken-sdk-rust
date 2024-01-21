use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

/// - <https://docs.kraken.com/rest/#tag/Funding/operation/withdrawFunds>
/// - <https://api.kraken.com/0/private/Withdraw>
#[must_use = "Does nothing until you send or execute it"]
pub struct WithdrawRequest {
    client: Client,
    /// Asset being withdrawn
    asset: String,
    /// Withdrawal key name, as set up on your account
    key: String,
    /// Amount to be withdrawn
    amount: String,
    /// Optional, crypto address that can be used to confirm address matches key (will return Invalid withdrawal address error if different)
    address: Option<String>,
    /// Optional, if the processed withdrawal fee is higher than max_fee, withdrawal will fail with EFunding:Max fee exceeded
    max_fee: Option<String>,
}

impl WithdrawRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let mut query = format!(
            "asset={}&key={}&amount={}",
            self.asset, self.key, self.amount,
        );

        if let Some(address) = &self.address {
            query.push_str(&format!("&address={}", address));
        }

        if let Some(max_fee) = &self.max_fee {
            query.push_str(&format!("&max_fee={}", max_fee));
        }

        self.client.send_private("/0/private/Withdraw", Some(query)).await
    }

    pub async fn send(self) -> Result<WithdrawResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct WithdrawResponse {
    pub result: WithdrawResult,
}

#[derive(Debug, Deserialize)]
pub struct WithdrawResult {
    pub refid: String,
}

impl Client {
    pub fn withdraw(&self, asset: &str, key: &str, amount: &str) -> WithdrawRequest {
        WithdrawRequest {
            client: self.clone(),
            asset: asset.to_string(),
            key: key.to_string(),
            amount: amount.to_string(),
            address: None,
            max_fee: None,
        }
    }
}
