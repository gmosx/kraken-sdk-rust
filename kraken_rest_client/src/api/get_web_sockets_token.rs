use crate::{Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

/// Returns a WebSocket API token. The token should be used within 15 minutes
/// of creation. The token does not expire once a connection to a WebSockets API
/// feed is maintained.
///
/// - <https://docs.kraken.com/rest/#operation/getWebsocketsToken>
/// - <https://api.kraken.com/0/private/GetWebSocketsToken>
#[must_use = "Does nothing until you send or execute it"]
pub struct GetWebSocketsTokenRequest {
    client: Client,
}

impl GetWebSocketsTokenRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        self.client
            .send_private("/0/private/GetWebSocketsToken", None)
            .await
    }

    pub async fn send(self) -> Result<GetWebSocketsTokenResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct GetWebSocketsTokenResponse {
    pub token: String,
    pub expires: i64, // TODO: better type?
}

impl Client {
    pub fn get_web_sockets_token(&self) -> GetWebSocketsTokenRequest {
        GetWebSocketsTokenRequest {
            client: self.clone(),
        }
    }
}
