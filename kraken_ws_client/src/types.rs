use std::default::Default;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    pub asset: String,
    pub qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum TimeInForce {
    /// Good-'til-cancelled is the default if the parameter is omitted.
    #[default]
    GTC,
    /// Good-'til-date. If specified, must coincide with a desired expire_time.
    GTD,
    /// Immediate-or-cancel will immediately execute the amount possible and
    /// cancel any remaining balance rather than resting in the book.
    IOC,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OrderType {
    Limit,
    Market,
    SettlePosition,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    New,
    Filled,
    Canceled,
    Expired,
    Triggered,
    PartiallyFilled,
    PendingNew,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Channel {
    Book,
    Executions,
    Instrument,
    Ticker,
    Trade,
    OHLC,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConditionalOrderType {
    Limit,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionalParams {
    pub limit_price: Option<f64>,
    pub stop_price: Option<f64>,
    pub order_type: Option<ConditionalOrderType>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum Depth {
    D10 = 10,
    D25 = 25,
    D100 = 100,
    D500 = 500,
    D1000 = 1000,
}
