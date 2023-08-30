use futures_util::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_stream::wrappers::BroadcastStream;

use crate::{
    client::{Event, PublicRequest},
    types::{Channel, Depth},
    Client,
};

// #todo synthesize the book
// #todo verify the checksum

#[derive(Debug, Serialize)]
pub struct SubscribeBookParams {
    pub channel: Channel,
    pub symbol: Vec<String>,
    /// Book depth for subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<Depth>,
    /// Request a snapshot after subscribing, default=true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

pub type SubscribeBookRequest = PublicRequest<SubscribeBookParams>;

impl SubscribeBookRequest {
    pub fn new(symbol: impl Into<Vec<String>>) -> Self {
        Self {
            method: "subscribe".into(),
            params: SubscribeBookParams {
                channel: Channel::Book,
                depth: None,
                symbol: symbol.into(),
                snapshot: None,
            },
            req_id: None,
        }
    }

    pub fn symbol(symbol: impl Into<String>) -> Self {
        Self::new(vec![symbol.into()])
    }

    pub fn depth(self, depth: Depth) -> Self {
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
    // #todo add support to filter for symbol.
    pub fn book_delta_events(&mut self) -> impl Stream<Item = BookEvent> {
        let messages_stream = BroadcastStream::new(self.messages.subscribe());

        let events_stream = messages_stream.filter_map(|msg| {
            std::future::ready(if let Ok(msg) = msg {
                serde_json::from_str::<BookEvent>(&msg).ok()
            } else {
                tracing::debug!("skipped {:?}", msg);
                None
            })
        });

        events_stream
    }
}
