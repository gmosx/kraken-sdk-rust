use std::fmt;

use serde::{Deserialize, Serialize};

pub type JsonValue = serde_json::Value;

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
