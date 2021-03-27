use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

/// - https://www.kraken.com/features/api#get-system-status
/// - https://api.kraken.com/0/public/SystemStatus
#[must_use = "Does nothing until you send or execute it"]
pub struct GetSystemStatusRequestBuilder {
    client: Client,
}

impl GetSystemStatusRequestBuilder {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        self.client.send_public("/0/public/SystemStatus").await
    }

    pub async fn send(self) -> Result<GetSystemStatusResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct GetSystemStatusResponse {
    /// Current system status or trading mode
    /// - online (operational, full trading available)
    /// - cancel_only (existing orders are cancelable, but new orders cannot be created)
    /// - post_only (existing orders are cancelable, and only new post limit orders can be submitted)
    /// - limit_only (existing orders are cancelable, and only new limit orders can be submitted)
    /// - maintenance (system is offline for maintenance)
    pub status: String,
    /// Server time
    pub timestamp: String,
}

impl Client {
    pub fn get_system_status(&self) -> GetSystemStatusRequestBuilder {
        GetSystemStatusRequestBuilder {
            client: self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, JsonValue, Result};

    #[test]
    fn get_system_status() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let client = Client::default();

            let resp = client.get_system_status().send().await;

            match resp {
                Ok(resp) => println!("{}", resp.status),
                Err(error) => eprintln!("{:?}", error),
            }

            let resp: Result<JsonValue> = client.get_system_status().execute().await;

            match resp {
                Ok(resp) => println!("{}", resp),
                Err(error) => eprintln!("{:?}", error),
            }
        });
    }
}
