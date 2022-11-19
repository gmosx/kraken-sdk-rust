use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    pub asset: String,
    pub qty: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TimeInForce {
    /// Good-'til-cancelled is the default if the parameter is omitted.
    GTC,
    /// Good-'til-date. If specified, must coincide with a desired expire_time.
    GTD,
    /// Immediate-or-cancel will immediately execute the amount possible and
    /// cancel any remaining balance rather than resting in the book.
    IOC,
}

impl Default for TimeInForce {
    fn default() -> Self {
        TimeInForce::GTC
    }
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
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    New,
    Filled,
    Canceled,
    Expired,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionName {
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
