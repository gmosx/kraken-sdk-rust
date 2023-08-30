use futures_util::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_stream::wrappers::BroadcastStream;

use crate::{
    client::{Event, PublicRequest},
    types::Channel,
    util::gen_next_id,
    Client,
};

#[derive(Debug, Serialize)]
pub struct SubscribeInstrumentParams {
    pub channel: Channel,
    /// Request a snapshot after subscribing, default=true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

/// - <https://docs.kraken.com/websockets-v2/#instrument>
pub type SubscribeInstrumentRequest = PublicRequest<SubscribeInstrumentParams>;

impl Default for SubscribeInstrumentRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl SubscribeInstrumentRequest {
    pub fn new() -> Self {
        Self {
            method: "subscribe".into(),
            params: SubscribeInstrumentParams {
                channel: Channel::Instrument,
                snapshot: None,
            },
            req_id: Some(gen_next_id()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetStatus {
    DepositOnly,
    Disabled,
    Enabled,
    FundingTemporarilyDisabled,
    WithdrawalOnly,
    WorkingProgress,
}

#[derive(Debug, Deserialize)]
pub struct Asset {
    pub borrowable: bool,
    pub collateral_value: f64,
    pub id: String,
    pub margin_rate: Option<f64>,
    pub precision: i32,
    pub precision_display: i32,
    pub status: AssetStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PairStatus {
    CancelOnly,
    Delisted,
    LimitOnly,
    Maintenance,
    Online,
    PostOnly,
    ReduceOnly,
    WorkingProgress,
}

#[derive(Debug, Deserialize)]
pub struct Pair {
    pub symbol: String,
    pub base: String,
    pub quote: String,
    pub cost_precision: i32,
    pub has_index: bool,
    pub margin_initial: Option<f64>,
    pub marginable: bool,
    pub position_limit_long: Option<i32>,
    pub position_limit_short: Option<i32>,
    pub price_increment: f64,
    pub price_precision: i32,
    pub qty_increment: f64,
    pub qty_min: f64,
    pub qty_precision: i32,
    pub status: PairStatus,
}

#[derive(Debug, Deserialize)]
pub struct InstrumentData {
    pub assets: Vec<Asset>,
    pub pairs: Vec<Pair>,
}

pub type InstrumentEvent = Event<InstrumentData>;

impl Client {
    pub fn instrument_events(&mut self) -> impl Stream<Item = InstrumentEvent> {
        let messages_stream = BroadcastStream::new(self.messages.subscribe());

        let events_stream = messages_stream.filter_map(|msg| {
            std::future::ready(if let Ok(msg) = msg {
                serde_json::from_str::<InstrumentEvent>(&msg).ok()
            } else {
                None
            })
        });

        events_stream
    }
}
