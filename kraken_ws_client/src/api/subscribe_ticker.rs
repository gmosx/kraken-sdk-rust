use futures_util::{Stream, StreamExt};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tokio_stream::wrappers::BroadcastStream;

use crate::{
    client::{Event, PublicRequest},
    types::Channel,
    util::gen_next_id,
    Client,
};

use super::SUBSCRIBE_METHOD;

#[derive(Debug, Serialize)]
pub struct SubscribeTickerParams {
    pub channel: Channel,
    pub symbol: Vec<String>,
    /// Request a snapshot after subscribing, default=true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

impl SubscribeTickerParams {
    pub fn new(symbol: impl Into<Vec<String>>) -> Self {
        Self {
            channel: Channel::Ticker,
            symbol: symbol.into(),
            snapshot: None,
        }
    }

    pub fn all() -> Self {
        Self {
            channel: Channel::Ticker,
            symbol: vec!["*".into()],
            snapshot: None,
        }
    }

    pub fn snapshot(self, snapshot: bool) -> Self {
        Self {
            snapshot: Some(snapshot),
            ..self
        }
    }
}

pub type SubscribeTickerRequest = PublicRequest<SubscribeTickerParams>;

impl SubscribeTickerRequest {
    pub fn new(params: SubscribeTickerParams) -> Self {
        Self {
            method: SUBSCRIBE_METHOD.into(),
            params,
            req_id: Some(gen_next_id()),
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

impl Client {
    // <https://docs.kraken.com/websockets-v2/#ticker>
    pub async fn subscribe_ticker(&mut self, symbol: impl Into<String>) {
        let symbol = vec![symbol.into()];
        self.subscribe_tickers(symbol).await
    }

    // <https://docs.kraken.com/websockets-v2/#ticker>
    pub async fn subscribe_tickers(&mut self, symbol: impl Into<Vec<String>>) {
        self.call_public(SUBSCRIBE_METHOD, SubscribeTickerParams::new(symbol))
            .await
            .expect("cannot send request");
    }

    // #todo add support to filter for symbol.
    pub fn ticker_events(&mut self) -> impl Stream<Item = TickerEvent> {
        let messages_stream = BroadcastStream::new(self.messages.subscribe());

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
