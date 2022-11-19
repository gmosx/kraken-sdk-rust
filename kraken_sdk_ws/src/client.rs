use crate::{util::Result};
use futures::{future, stream::SplitSink, StreamExt};
use futures_util::SinkExt;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio::net::TcpStream;
use tokio_stream::Stream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

pub const WS_URL: &str = "wss://ws.kraken.com/v2";
pub const WS_AUTH_URL: &str = "wss://ws-auth.kraken.com/v2";

pub const DEFAULT_WS_URL: &str = WS_URL;
pub const DEFAULT_WS_AUTH_URL: &str = WS_AUTH_URL;

#[derive(Debug, Serialize)]
pub struct Request<P> {
    pub method: String,
    pub params: P,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct Response<R> {
    pub method: String,
    pub req_id: Option<i64>,
    pub result: R,
    pub success: bool,
    pub time_in: String,
    pub time_out: String,
}

// #TODO consider renaming SubscriptionEvent or ChannelEvent.
#[derive(Debug, Deserialize)]
pub struct Event<D> {
    pub channel: String,
    pub data: D,
    #[serde(rename = "type")]
    pub event_type: String,
}

// #TODO not used yet!
#[derive(Serialize, Deserialize, Debug)]
pub enum TypedMessage {
    Other(String),
}

// Subscription messages (event) have the `channel` field.
// RPC messages (response) have the `method` field.
// Error messages have the `error` field.

/// A WebSocket client for Kraken.
/// The client can connect to a `public` endpoint or an `auth` endpoint.
/// The `auth` endpoint only supports auth messages.
pub struct Client {
    #[allow(dead_code)]
    token: Option<String>,
    sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    pub messages: Pin<Box<dyn Stream<Item = Result<TypedMessage>>>>,
}

// #TODO extract socket like in the previous impl?
impl Client {
    pub async fn connect(url: &str, token: Option<String>) -> Result<Self> {
        let (stream, _) = connect_async(url).await?;
        let (sender, receiver) = stream.split();

        let receiver = receiver
            .map(move |msg| {
                if let Message::Text(string) = msg.unwrap() {
                    tracing::debug!("{string}");
                    Ok(Some(TypedMessage::Other(string)))
                } else {
                    Ok(None)
                }
            })
            .filter_map(|res| future::ready(res.transpose()));

        Ok(Self {
            token,
            sender,
            messages: Box::pin(receiver),
        })
    }

    pub async fn connect_public() -> Result<Self> {
        Self::connect(WS_URL, None).await
    }

    pub async fn connect_auth(token: String) -> Result<Self> {
        Self::connect(WS_AUTH_URL, Some(token)).await
    }

    pub async fn send<Req>(&mut self, req: Req) -> Result<()>
        where Req: Serialize {
        // #TODO attach the token to the request here! nah!
        // // #TODO add rec_id
        let msg = serde_json::to_string(&req).unwrap();
        tracing::debug!("{msg}");
        self.sender.send(Message::Text(msg.to_string())).await?;

        Ok(())
    }

    // #TODO make this customizable.
    pub fn next_id(&self) -> isize {
        todo!()
    }
}
