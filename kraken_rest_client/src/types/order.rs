use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Ord, PartialOrd, Hash)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let order_side = match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        };

        write!(f, "{}", order_side)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Ord, PartialOrd, Hash)]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    TakeProfit,
    StopLossLimit,
    TakeProfitLimit,
    SettlePosition,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let order_type = match self {
            Self::Market => "market",
            Self::Limit => "limit",
            Self::StopLoss => "stop-loss",
            Self::TakeProfit => "take-profit",
            Self::StopLossLimit => "stop-loss-limit",
            Self::TakeProfitLimit => "take-profit-limit",
            Self::SettlePosition => "settle-position",
        };

        write!(f, "{}", order_type)
    }
}

#[derive(Debug, Deserialize)]
pub struct OrderDescription {
    // pub ordertxid: Option<String>,
    // pub postxid: Option<String>,
    pub pair: String,
    // pub time: f64,
    #[serde(rename = "type")]
    pub orderside: String,
    pub ordertype: String,
    pub price: String,
    pub price2: String,
    pub leverage: String,
    pub order: String,
    pub close: String,
    // TODO: the following fields are missing in some orders:
    // pub cost: String,
    // pub fee: String,
    // pub vol: String,
    // pub margin: String,
    // pub misc: String,
}
