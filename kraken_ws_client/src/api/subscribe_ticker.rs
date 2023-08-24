use async_stream::stream;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    client::{Event, Request},
    types::Channel,
    Client,
};

#[derive(Debug, Serialize)]
pub struct SubscribeTickerParams<'a> {
    pub channel: Channel,
    pub symbol: &'a [&'a str],
    /// Request a snapshot after subscribing, default=true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

/// - <https://docs.kraken.com/websockets-v2/#ticker>
pub type SubscribeTickerRequest<'a> = Request<SubscribeTickerParams<'a>>;

impl SubscribeTickerRequest<'_> {
    pub fn new<'a>(symbol: &'a [&'a str]) -> SubscribeTickerRequest<'a> {
        SubscribeTickerRequest {
            method: "subscribe".to_owned(),
            params: SubscribeTickerParams {
                channel: Channel::Ticker,
                symbol,
                snapshot: None,
            },
            req_id: None,
        }
    }

    pub fn all<'a>() -> SubscribeTickerRequest<'a> {
        SubscribeTickerRequest {
            method: "subscribe".to_owned(),
            params: SubscribeTickerParams {
                channel: Channel::Ticker,
                symbol: &["*"],
                snapshot: None,
            },
            req_id: None,
        }
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

impl Client {
    // <https://docs.kraken.com/websockets-v2/#ticker>
    pub async fn subscribe_ticker(&mut self, symbol: impl AsRef<str>) {
        let symbol = &[symbol.as_ref()];
        self.subscribe_tickers(symbol).await
    }

    // <https://docs.kraken.com/websockets-v2/#ticker>
    pub async fn subscribe_tickers(&mut self, symbol: &[&str]) {
        let req = SubscribeTickerRequest::new(symbol);

        self.send(req).await.expect("cannot send request");

        let mut messages_receiver = self.broadcast.clone().subscribe();

        let ticker_events = stream! {
            while let Ok(msg) = messages_receiver.recv().await {
                if let Ok(msg) = serde_json::from_str::<TickerEvent>(&msg) {
                    yield msg
                }
            }
        };

        self.ticker_events = Some(Box::pin(ticker_events));
    }
}
