use std::pin::Pin;

use futures::{stream::SplitSink, StreamExt};
use futures_util::{SinkExt, Stream};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

use crate::{
    api::{BookEvent, TickerEvent},
    util::Result,
};

pub const DEFAULT_WS_URL: &str = "wss://ws.kraken.com/v2";
pub const DEFFAULT_WS_AUTH_URL: &str = "wss://ws-auth.kraken.com/v2";

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
    sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,

    // The thread_handle will be dropped when the Client drops.
    #[allow(dead_code)]
    thread_handle: tokio::task::JoinHandle<()>,
    pub broadcast: tokio::sync::broadcast::Sender<String>,
    pub book_events: Option<Pin<Box<dyn Stream<Item = BookEvent> + Send + Sync>>>,
    pub ticker_events: Option<Pin<Box<dyn Stream<Item = TickerEvent> + Send + Sync>>>,
}

// #todo extract socket like in the previous impl?
impl Client {
    pub async fn connect(url: &str, token: Option<String>) -> Result<Self> {
        let (stream, _) = connect_async(url).await?;
        let (sender, receiver) = stream.split();
        let (broadcast_sender, _) = tokio::sync::broadcast::channel::<String>(32);

        let broadcast = broadcast_sender.clone();

        let thread_handle = tokio::spawn(async move {
            let mut receiver = receiver;

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
            sender,
            thread_handle,
            broadcast,
            book_events: None,
            ticker_events: None,
        })
    }

    pub async fn connect_public() -> Result<Self> {
        Self::connect(DEFAULT_WS_URL, None).await
    }

    pub async fn connect_auth(token: String) -> Result<Self> {
        Self::connect(DEFFAULT_WS_AUTH_URL, Some(token)).await
    }

    pub async fn send<Req>(&mut self, req: Req) -> Result<()>
    where
        Req: Serialize,
    {
        // #todo attach the token to the request here! nah!
        // #todo add rec_id
        let msg = serde_json::to_string(&req).unwrap();
        tracing::debug!("{msg}");
        self.sender.send(Message::Text(msg.to_string())).await?;

        Ok(())
    }

    // #todo make this customizable.
    pub fn next_id(&self) -> isize {
        todo!()
    }
}
