use futures_util::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_stream::wrappers::BroadcastStream;

use crate::{
    client::{Event, Request},
    types::{BookDepth, Channel},
    util::gen_next_id,
    Client,
};

use super::SUBSCRIBE_METHOD;

// #todo synthesize the book
// #todo verify the checksum

#[derive(Debug, Serialize)]
pub struct SubscribeBookParams {
    pub channel: Channel,
    pub symbol: Vec<String>,
    /// Book depth for subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<BookDepth>,
    /// Request a snapshot after subscribing, default=true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

impl SubscribeBookParams {
    pub fn new<'a>(symbol: impl Into<Vec<String>>) -> Self {
        Self {
            channel: Channel::Book,
            depth: None,
            symbol: symbol.into(),
            snapshot: None,
        }
    }

    pub fn depth(self, depth: BookDepth) -> Self {
        Self {
            depth: Some(depth),
            ..self
        }
    }

    pub fn snapshot(self, snapshot: bool) -> Self {
        Self {
            snapshot: Some(snapshot),
            ..self
        }
    }
}

pub type SubscribeBookRequest = Request<SubscribeBookParams>;

impl SubscribeBookRequest {
    pub fn new(params: SubscribeBookParams) -> Self {
        Self {
            method: SUBSCRIBE_METHOD.into(),
            params,
            req_id: Some(gen_next_id()),
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
    pub async fn subscribe_book(&mut self, symbol: impl Into<String>, depth: BookDepth) {
        let symbol = vec![symbol.into()];
        self.subscribe_books(symbol, depth).await
    }

    // <https://docs.kraken.com/websockets-v2/#book>
    pub async fn subscribe_books(&mut self, symbol: impl Into<Vec<String>>, depth: BookDepth) {
        self.call(
            SUBSCRIBE_METHOD,
            SubscribeBookParams::new(symbol).depth(depth),
        )
        .await
        .expect("cannot send request");
    }

    // #todo add support to filter for symbol.
    pub fn book_delta_events(&mut self) -> impl Stream<Item = BookEvent> {
        let messages_stream = BroadcastStream::new(self.broadcast.subscribe());

        let events_stream = messages_stream.filter_map(|msg| {
            std::future::ready(if let Ok(msg) = msg {
                serde_json::from_str::<BookEvent>(&msg).ok()
            } else {
                None
            })
        });

        events_stream
    }
}
