use crate::{Client, Result};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// - https://www.kraken.com/features/api#get-account-balance
/// - https://api.kraken.com/0/private/Balance
#[must_use = "Does nothing until you send or execute it"]
pub struct GetAccountBalanceRequestBuilder {
    client: Client,
}

impl GetAccountBalanceRequestBuilder {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        self.client.send_private("/0/private/Balance", None).await
    }

    pub async fn send(self) -> Result<GetAccountBalanceResponse> {
        self.execute().await
    }
}

pub type GetAccountBalanceResponse = HashMap<String, String>;

impl Client {
    pub fn get_account_balance(&self) -> GetAccountBalanceRequestBuilder {
        GetAccountBalanceRequestBuilder {
            client: self.clone(),
        }
    }
}
