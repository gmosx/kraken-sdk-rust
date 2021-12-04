use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::Url;

pub const WS_URL: &str = "wss://ws.kraken.com";
pub const WS_AUTH_URL: &str = "wss://ws-auth.kraken.com";
pub const BETA_WS_URL: &str = "wss://beta-ws.kraken.com";
pub const BETA_WS_AUTH_URL: &str = "wss://beta-ws-auth.kraken.com";

// TODO: add support for getting the auth token!

#[derive(Debug)]
pub struct Socket {
    pub url: Url,
    pub token: Option<String>,
    pub stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl Socket {
    // TODO: accept something like 'IntoUrl'
    pub async fn connect(url: &str, token: Option<String>) -> Self {
        let (stream, _) = connect_async(url).await.expect("Failed to connect");

        Self {
            url: url::Url::parse(url).unwrap(),
            token,
            stream,
        }
    }

    pub async fn connect_public() -> Self {
        Self::connect(WS_URL, None).await
    }

    pub async fn connect_auth(token: &str) -> Self {
        Self::connect(WS_AUTH_URL, Some(token.to_owned())).await
    }

    pub async fn send(
        &mut self,
        msg: Message,
    ) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        self.stream.send(msg).await
    }

    pub async fn next(&mut self) -> Option<Result<Message, tokio_tungstenite::tungstenite::Error>> {
        self.stream.next().await
    }
}
