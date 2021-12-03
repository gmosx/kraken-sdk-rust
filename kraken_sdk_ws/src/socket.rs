use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;

pub const WS_URL: &str = "wss://ws.kraken.com";
pub const WS_AUTH_URL: &str = "wss://ws-auth.kraken.com";
pub const BETA_WS_URL: &str = "wss://beta-ws.kraken.com";
pub const BETA_WS_AUTH_URL: &str = "wss://beta-ws-auth.kraken.com";

// TODO: add support for handling getting the token!

#[derive(Debug)]
pub struct Socket {
    pub url: Url,
    pub token: Option<String>,
    pub stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl Socket {
    // TODO: accept something like 'IntoUrl'
    pub fn new(url: &str, token: &str) -> Self {
        Self {
            url: url::Url::parse(url).unwrap(),
            token: Some(token.to_owned()),
            stream: None,
        }
    }

    pub fn public() -> Self {
        Self {
            url: url::Url::parse(WS_URL).unwrap(),
            token: None,
            stream: None,
        }
    }

    pub fn auth(token: &str) -> Self {
        Self {
            url: url::Url::parse(WS_AUTH_URL).unwrap(),
            token: Some(token.to_owned()),
            stream: None,
        }
    }

    pub async fn connect(&mut self) {
        let (stream, _) = connect_async(&self.url).await.expect("Failed to connect");
        self.stream = Some(stream);
    }
}
