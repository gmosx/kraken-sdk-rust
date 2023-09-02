use futures_util::{Stream, StreamExt};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tokio_stream::wrappers::BroadcastStream;

use crate::{
    client::{Event, PublicClient, PublicRequest},
    types::Channel,
};

#[derive(Debug, Serialize)]
pub struct SubscribeTickerParams {
    pub channel: Channel,
    pub symbol: Vec<String>,
    /// Request a snapshot after subscribing, default=true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

pub type SubscribeTickerRequest = PublicRequest<SubscribeTickerParams>;

impl SubscribeTickerRequest {
    pub fn new(symbol: impl Into<Vec<String>>) -> Self {
        Self {
            method: "subscribe".into(),
            params: SubscribeTickerParams {
                channel: Channel::Ticker,
                symbol: symbol.into(),
                snapshot: None,
            },
            req_id: None,
        }
    }

    pub fn symbol(symbol: impl Into<String>) -> Self {
        Self::new(vec![symbol.into()])
    }

    pub fn all() -> Self {
        Self::symbol("*")
    }

    pub fn snapshot(self, snapshot: bool) -> Self {
        Self {
            params: SubscribeTickerParams {
                snapshot: Some(snapshot),
                ..self.params
            },
            ..self
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TickerData {
    pub ask: Decimal,
    pub ask_qty: Decimal,
    pub bid: Decimal,
    pub bid_qty: Decimal,
    pub change: Decimal,
    pub change_pct: Decimal,
    pub high: Decimal,
    pub last: Decimal,
    pub low: Decimal,
    pub symbol: String,
    pub volume: Decimal,
    pub vwap: Decimal,
}

pub type TickerEvent = Event<Vec<TickerData>>;

impl PublicClient {
    // #todo add support to filter for symbol.
    pub fn ticker_events(&mut self) -> impl Stream<Item = TickerEvent> {
        let messages_stream = BroadcastStream::new(self.messages());

        let events_stream = messages_stream.filter_map(|msg| {
            std::future::ready(if let Ok(msg) = msg {
                serde_json::from_str::<TickerEvent>(&msg).ok()
            } else {
                None
            })
        });

        events_stream
    }
}
