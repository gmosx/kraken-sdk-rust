use futures::{stream::SplitSink, StreamExt};
use futures_util::SinkExt;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

use crate::util::{gen_next_id, Result};

pub const DEFAULT_WS_URL: &str = "wss://ws.kraken.com/v2";
pub const DEFFAULT_WS_AUTH_URL: &str = "wss://ws-auth.kraken.com/v2";

// #todo create PrivateRequest, with token?

#[derive(Debug, Serialize)]
pub enum Request<P: Serialize> {
    Public(PublicRequest<P>),
    Private(PrivateRequest<P>),
}

#[derive(Debug, Serialize)]
pub struct PublicRequest<P: Serialize> {
    pub method: String,
    pub params: P,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct PrivateParams<P: Serialize> {
    #[serde(flatten)]
    pub params: P,
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct PrivateRequest<P: Serialize> {
    pub method: String,
    pub params: PrivateParams<P>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,
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

// #todo consider renaming SubscriptionEvent or ChannelEvent.
#[derive(Debug, Deserialize)]
pub struct Event<D> {
    pub channel: String,
    pub data: D,
    #[serde(rename = "type")]
    pub event_type: String,
}

// Subscription messages (event) have the `channel` field.
// RPC messages (response) have the `method` field.
// Error messages have the `error` field.

/// A WebSocket client for Kraken.
///
/// The client can connect to a `public` endpoint or an `auth` endpoint.
/// The `auth` endpoint only supports auth messages.
pub struct Client {
    #[allow(dead_code)]
    token: Option<String>,
    websocket_sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    // The thread_handle will be dropped when the Client drops.
    #[allow(dead_code)]
    thread_handle: tokio::task::JoinHandle<()>,
    pub messages: tokio::sync::broadcast::Sender<String>,
}

// #todo extract socket like in the previous impl?
// #todo separate handling of Response, Event, Error.

impl Client {
    pub async fn connect(url: &str, token: Option<String>) -> Result<Self> {
        let (websocket_stream, _) = connect_async(url).await?;
        let (websocket_sender, websocket_receiver) = websocket_stream.split();
        let (broadcast_sender, _) = tokio::sync::broadcast::channel::<String>(32);

        let broadcast_sender_clone = broadcast_sender.clone();

        let thread_handle = tokio::spawn(async move {
            let mut receiver = websocket_receiver;

            while let Some(result) = receiver.next().await {
                if let Ok(msg) = result {
                    if let Message::Text(string) = msg {
                        tracing::debug!("{string}");
                        if let Err(err) = broadcast_sender.send(string) {
                            tracing::trace!("{err:?}");
                            // Break the while loop so that the receiver handle is dropped
                            // and the task unsubscribes from the summary stream.
                            break;
                        }
                    }
                } else {
                    tracing::error!("{:?}", result);
                }
            }
        });

        Ok(Self {
            token,
            websocket_sender,
            thread_handle,
            messages: broadcast_sender_clone,
        })
    }

    pub async fn connect_public() -> Result<Self> {
        Self::connect(DEFAULT_WS_URL, None).await
    }

    pub async fn connect_auth(token: String) -> Result<Self> {
        Self::connect(DEFFAULT_WS_AUTH_URL, Some(token)).await
    }

    /// Sends a message to the WebSocket.
    pub async fn send<Req>(&mut self, req: Req) -> Result<()>
    where
        Req: Serialize,
    {
        let msg = serde_json::to_string(&req).unwrap();

        tracing::debug!("{msg}");

        self.websocket_sender
            .send(Message::Text(msg.to_string()))
            .await?;

        Ok(())
    }

    /// Performs a public remote procedure call.
    pub async fn call_public<P>(&mut self, method: impl Into<String>, params: P) -> Result<()>
    where
        P: Serialize,
    {
        // #todo attach the token to the request here! nah!
        let req = PublicRequest {
            method: method.into(),
            params,
            req_id: Some(gen_next_id()),
        };

        self.send(req).await
    }

    /// Performs a private remote procedure call.
    pub async fn call_private<P>(&mut self, method: impl Into<String>, params: P) -> Result<()>
    where
        P: Serialize,
    {
        let req = PrivateRequest {
            method: method.into(),
            params: PrivateParams {
                params: params,
                token: self.token.clone().unwrap(),
            },
            req_id: Some(gen_next_id()),
        };

        self.send(req).await
    }
}
