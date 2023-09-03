use futures::{stream::SplitSink, StreamExt};
use futures_util::SinkExt;
use serde::{Deserialize, Serialize};
use tokio::{net::TcpStream, sync::broadcast::Receiver};
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

use crate::util::{gen_next_id, Result};

pub const DEFAULT_WS_URL: &str = "wss://ws.kraken.com/v2";
pub const DEFFAULT_WS_PRIVATE_URL: &str = "wss://ws-auth.kraken.com/v2";

#[derive(Debug, Serialize)]
pub struct PublicRequest<P: Serialize> {
    pub method: String,
    pub params: P,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,
}

impl<P: Serialize> PublicRequest<P> {
    pub fn req_id(self, req_id: u64) -> Self {
        Self {
            req_id: Some(req_id),
            ..self
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PrivateParams<P: Serialize> {
    #[serde(flatten)]
    pub params: P,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl<P: Serialize> PrivateParams<P> {
    pub fn new(params: P) -> Self {
        Self {
            params,
            token: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PrivateRequest<P: Serialize> {
    pub method: String,
    pub params: PrivateParams<P>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,
}

impl<P: Serialize> PrivateRequest<P> {
    pub fn token(self, token: impl Into<String>) -> Self {
        Self {
            params: PrivateParams {
                token: Some(token.into()),
                ..self.params
            },
            ..self
        }
    }
    pub fn req_id(self, req_id: u64) -> Self {
        Self {
            req_id: Some(req_id),
            ..self
        }
    }
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

// #todo find a better name: Backend, Driver.
// #todo separate connect and spawn.

/// A WebSocket transport for Kraken.
///
/// Can connect to a `public` endpoint or an `auth` endpoint.
/// The `auth` endpoint only supports auth messages.
pub struct Transport {
    websocket_sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    // The thread_handle will be dropped when the Client drops.
    #[allow(dead_code)]
    thread_handle: tokio::task::JoinHandle<()>,
    pub messages: tokio::sync::broadcast::Sender<String>,
}

// #todo extract socket like in the previous impl?
// #todo separate handling of Response, Event, Error.

impl Transport {
    pub async fn connect(url: &str) -> Result<Self> {
        let (websocket_stream, _) = connect_async(url).await?;
        let (websocket_sender, websocket_receiver) = websocket_stream.split();
        let (broadcast_sender, _) = tokio::sync::broadcast::channel::<String>(32);

        let broadcast_sender_clone = broadcast_sender.clone();

        let thread_handle = tokio::spawn(async move {
            websocket_receiver
                .for_each(|result| async {
                    // while let Some(result) = receiver.next().await {
                    if let Ok(msg) = result {
                        if let Message::Text(string) = msg {
                            tracing::debug!("{string}");
                            if let Err(err) = broadcast_sender.send(string) {
                                // A send operation can only fail if there are no
                                // active receivers, implying that the message could
                                // never be received.

                                // #todo we skip the message but should probably buffer it?

                                tracing::trace!("{err:?}");
                                // #insight
                                // We don't do that any more:
                                // Break the while loop so that the receiver handle is dropped
                                // and the task unsubscribes from the summary stream.

                                // #todo intentionally don't break from the loop.
                                // break;
                            }
                        } else {
                            tracing::debug!("unexpected message '{msg}'");
                        }
                    } else {
                        tracing::error!("{:?}", result);
                    }
                })
                .await;
        });

        Ok(Self {
            websocket_sender,
            thread_handle,
            messages: broadcast_sender_clone,
        })
    }

    /// Sends a public message to the WebSocket.
    async fn send<R>(&mut self, req: R) -> Result<()>
    where
        R: Serialize,
    {
        let msg = serde_json::to_string(&req).unwrap();

        tracing::debug!("{msg}");

        self.websocket_sender
            .send(Message::Text(msg.to_string()))
            .await?;

        Ok(())
    }
}

pub struct PublicClient {
    transport: Transport,
}

impl PublicClient {
    pub async fn connect() -> Result<Self> {
        Ok(Self {
            transport: Transport::connect(DEFAULT_WS_URL).await?,
        })
    }

    pub async fn send<P>(&mut self, req: PublicRequest<P>) -> Result<()>
    where
        P: Serialize,
    {
        let mut req = req;

        if req.req_id.is_none() {
            req.req_id = Some(gen_next_id());
        }

        self.transport.send(req).await
    }

    pub fn messages(&mut self) -> Receiver<String> {
        self.transport.messages.subscribe()
    }
}

pub struct PrivateClient {
    transport: Transport,
    token: String,
}

impl PrivateClient {
    pub async fn connect(token: impl Into<String>) -> Result<Self> {
        Ok(Self {
            transport: Transport::connect(DEFFAULT_WS_PRIVATE_URL).await?,
            token: token.into(),
        })
    }

    pub async fn send<P>(&mut self, req: PrivateRequest<P>) -> Result<()>
    where
        P: Serialize,
    {
        let mut req = req;

        req.params.token = Some(self.token.clone());

        if req.req_id.is_none() {
            req.req_id = Some(gen_next_id());
        }

        self.transport.send(req).await
    }

    pub fn messages(&mut self) -> Receiver<String> {
        self.transport.messages.subscribe()
    }
}
