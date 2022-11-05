use crate::util::Result;
use futures::{future, stream::SplitSink, StreamExt};
use futures_util::SinkExt;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio::net::TcpStream;
use tokio_stream::Stream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};
use uuid::Uuid;

pub const WS_URL: &str = "wss://ws.kraken.com";
pub const WS_AUTH_URL: &str = "wss://ws-auth.kraken.com";
pub const BETA_WS_URL: &str = "wss://beta-ws.kraken.com";
pub const BETA_WS_AUTH_URL: &str = "wss://beta-ws-auth.kraken.com";

pub const DEFAULT_WS_URL: &str = WS_URL;

#[derive(Serialize, Deserialize, Debug)]
pub enum TypedMessage {
    Other(String),
}

/// A WebSocket client for Kraken.
pub struct Client {
    sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    pub messages: Pin<Box<dyn Stream<Item = Result<TypedMessage>>>>,
}

impl Client {
    pub async fn connect(url: &str) -> Result<Self> {
        let (stream, _response) = connect_async(url).await?;
        let (sender, receiver) = stream.split();

        let receiver = receiver
            .map(move |msg| {
                if let Message::Text(string) = msg.unwrap() {
                    Ok(Some(TypedMessage::Other(string)))
                } else {
                    Ok(None)
                }
            })
            .filter_map(|res| future::ready(res.transpose()));

        Ok(Self {
            sender,
            messages: Box::pin(receiver),
        })
    }

    pub async fn call<Req>(&mut self, req: Req) -> Result<()>
    where
        Req: Serialize,
    {
        let msg = serde_json::to_value(&req).unwrap(); // #TODO use `?`.

        // #TODO, this is temp code, add error-handling!

        if let serde_json::Value::Object(map) = msg {
            let msg = serde_json::to_string(&map).unwrap();

            self.sender.send(Message::Text(msg.to_string())).await?;
        }

        Ok(())
    }

    // #TODO make this customizable.
    pub fn next_id(&self) -> String {
        Uuid::new_v4().to_string()
    }
}
