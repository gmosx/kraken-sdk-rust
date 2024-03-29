use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

/// - <https://www.kraken.com/features/api#get-server-time>
/// - <https://api.kraken.com/0/public/Time>
#[must_use = "Does nothing until you send or execute it"]
pub struct GetServerTimeRequest {
    client: Client,
}

impl GetServerTimeRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        self.client.send_public("/0/public/Time").await
    }

    pub async fn send(self) -> Result<GetServerTimeResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct GetServerTimeResponse {
    pub unixtime: i64,
    pub rfc1123: String,
}

impl Client {
    pub fn get_server_time(&self) -> GetServerTimeRequest {
        GetServerTimeRequest {
            client: self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, JsonValue, Result};

    #[tokio::test]
    async fn get_server_time() {
        let client = Client::default();

        let resp = client.get_server_time().send().await;

        match resp {
            Ok(resp) => println!("{}", resp.unixtime),
            Err(error) => eprintln!("{:?}", error),
        }

        let resp: Result<JsonValue> = client.get_server_time().execute().await;

        match resp {
            Ok(resp) => println!("{}", resp),
            Err(error) => eprintln!("{:?}", error),
        }
    }
}
