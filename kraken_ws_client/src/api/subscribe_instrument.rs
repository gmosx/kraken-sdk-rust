use crate::{
    client::{Event, Request},
    types::Channel,
    util::gen_next_id,
};
use serde::{Deserialize, Serialize};

use super::SUBSCRIBE_METHOD;

#[derive(Debug, Serialize)]
pub struct SubscribeInstrumentParams {
    pub channel: Channel,
    /// Request a snapshot after subscribing, default=true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

impl Default for SubscribeInstrumentParams {
    fn default() -> Self {
        Self::new()
    }
}

impl SubscribeInstrumentParams {
    pub fn new() -> Self {
        Self {
            channel: Channel::Instrument,
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

/// - <https://docs.kraken.com/websockets-v2/#instrument>
pub type SubscribeInstrumentRequest = Request<SubscribeInstrumentParams>;

impl SubscribeInstrumentRequest {
    pub fn new(params: SubscribeInstrumentParams) -> Self {
        Self {
            method: SUBSCRIBE_METHOD.into(),
            params,
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
