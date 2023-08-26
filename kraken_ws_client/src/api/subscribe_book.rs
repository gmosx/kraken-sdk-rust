use async_stream::stream;
use serde::{Deserialize, Serialize};

use crate::{
    client::{Event, Request},
    types::{BookDepth, Channel},
    Client,
};

#[derive(Debug, Serialize)]
pub struct SubscribeBookParams<'a> {
    pub channel: Channel,
    pub symbol: &'a [&'a str],
    /// Book depth for subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<BookDepth>,
    /// Request a snapshot after subscribing, default=true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

/// - <https://docs.kraken.com/websockets-v2/#book>
pub type SubscribeBookRequest<'a> = Request<SubscribeBookParams<'a>>;

impl SubscribeBookRequest<'_> {
    pub fn new<'a>(symbol: &'a [&'a str]) -> SubscribeBookRequest<'a> {
        SubscribeBookRequest {
            method: "subscribe".to_owned(),
            params: SubscribeBookParams {
                channel: Channel::Book,
                depth: None,
                symbol,
                snapshot: None,
            },
            req_id: None,
        }
    }

    pub fn depth(self, depth: BookDepth) -> Self {
        Self {
            params: SubscribeBookParams {
                depth: Some(depth),
                ..self.params
            },
            ..self
        }
    }

    pub fn snapshot(self, snapshot: bool) -> Self {
        Self {
            params: SubscribeBookParams {
                snapshot: Some(snapshot),
                ..self.params
            },
            ..self
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LevelData {
    pub price: f64,
    pub qty: f64,
}

#[derive(Debug, Deserialize)]
pub struct BookData {
    pub bids: Vec<LevelData>,
    pub asks: Vec<LevelData>,
    pub checksum: u32,
    pub symbol: String,
}

pub type BookEvent = Event<Vec<BookData>>;

impl Client {
    // <https://docs.kraken.com/websockets-v2/#book>
    pub async fn subscribe_book(&mut self, symbol: impl AsRef<str>, depth: BookDepth) {
        let symbol = &[symbol.as_ref()];
        self.subscribe_books(symbol, depth).await
    }

    // <https://docs.kraken.com/websockets-v2/#book>
    pub async fn subscribe_books(&mut self, symbol: &[&str], depth: BookDepth) {
        let req = SubscribeBookRequest::new(symbol).depth(depth);

        self.send(req).await.expect("cannot send request");

        let mut messages_receiver = self.broadcast.clone().subscribe();

        let book_events = stream! {
            while let Ok(msg) = messages_receiver.recv().await {
               if let Ok(msg) = serde_json::from_str::<BookEvent>(&msg) {
                    yield msg
                }
                // #todo not good!
            }
        };

        self.book_events = Some(Box::pin(book_events));
    }
}
