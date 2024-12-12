use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error;
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

impl TryFrom<String> for OrderSide {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "sell" => Ok(OrderSide::Sell),
            "buy" => Ok(OrderSide::Buy),
            _ => Err(format!("Invalid order type: {}", s)),
        }
    }
}

fn orderside_from_str<'de, D>(deserializer: D) -> Result<OrderSide, D::Error> where D: Deserializer<'de>, {
    let orderside = String::deserialize(deserializer)?;
    OrderSide::try_from(orderside).map_err(D::Error::custom)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Ord, PartialOrd, Hash)]
pub enum OrderType {
    Market,
    Limit,
    Iceberg,
    StopLoss,
    TakeProfit,
    StopLossLimit,
    TakeProfitLimit,
    SettlePosition,
    TrailingStop,
    TrailingStopLimit

}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let order_type = match self {
            Self::Market => "market",
            Self::Limit => "limit",
            Self::Iceberg => "iceberg",
            Self::StopLoss => "stop-loss",
            Self::TakeProfit => "take-profit",
            Self::StopLossLimit => "stop-loss-limit",
            Self::TakeProfitLimit => "take-profit-limit",
            Self::SettlePosition => "settle-position",
            Self::TrailingStop => "trailing-stop",
            Self::TrailingStopLimit => "trailing-stop-limit"
        };

        write!(f, "{}", order_type)
    }
}

impl TryFrom<String> for OrderType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "market" => Ok(Self::Market),
            "limit" => Ok(Self::Limit),
            "iceberg" => Ok(Self::Iceberg),
            "stop-loss" => Ok(Self::StopLoss),
            "take-profit" => Ok(Self::TakeProfit),
            "stop-loss-limit" => Ok(Self::StopLossLimit),
            "take-profit-limit" => Ok(Self::TakeProfitLimit),
            "settle-position" => Ok(Self::SettlePosition),
            "trailing-stop" => Ok(Self::TrailingStop),
            "trailing-stop-limit" => Ok(Self::TrailingStopLimit),
            _ => Err(format!("Invalid order type: {}", s)),
        }
    }
}

fn ordertype_from_str<'de, D>(deserializer: D) -> Result<OrderType, D::Error> where D: Deserializer<'de>, {
    let ordertype = String::deserialize(deserializer)?;
    OrderType::try_from(ordertype).map_err(D::Error::custom)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderDescription {
    // pub ordertxid: Option<String>,
    // pub postxid: Option<String>,
    pub pair: String,
    // pub time: f64,
    #[serde(rename = "type")]
    #[serde(deserialize_with = "orderside_from_str")]
    pub orderside: OrderSide,
    #[serde(deserialize_with = "ordertype_from_str")]
    pub ordertype: OrderType,
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
