use crate::{client::Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

pub enum PriceType {
    Trade,
    Mark,
    Spot,
}

impl std::fmt::Display for PriceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Trade => write!(f, "trade"),
            Self::Mark => write!(f, "mark"),
            Self::Spot => write!(f, "spot"),
        }
    }
}

/// Time interval for candle.
pub enum Interval {
    Min1,
    Min5,
    Min15,
    Min30,
    Hour1,
    Hour4,
    Hour12,
    Day1,
    Week1,
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Min1 => write!(f, "1m"),
            Self::Min5 => write!(f, "5m"),
            Self::Min15 => write!(f, "15m"),
            Self::Min30 => write!(f, "30m"),
            Self::Hour1 => write!(f, "1h"),
            Self::Hour4 => write!(f, "4h"),
            Self::Hour12 => write!(f, "12h"),
            Self::Day1 => write!(f, "1d"),
            Self::Week1 => write!(f, "1w"),
        }
    }
}

/// This endpoint returns the trade, mark or spot price Open, High, Low, Close
/// (OHLC) candle data for the most recent candle or from a specified time.
///
/// Note: the maximum number of candles returned is 5000.
///
/// ## Documentation
///
/// - https://support.kraken.com/hc/en-us/articles/4403284627220-OHLC
/// - https://support.kraken.com/hc/en-us/articles/360022835891-Ticker-symbols
///
/// ## Sample calls
///
/// - https://futures.kraken.com/api/charts/v1/trade/PI_XBTUSD/1m
/// - https://futures.kraken.com/api/charts/v1/spot/PI_XBTUSD/1m?from=1625405796
/// - https://futures.kraken.com/api/charts/v1/mark/PI_XBTUSD/1m?from=1625405796&to=1625492256
#[must_use = "Does nothing until you send or execute it"]
pub struct GetOhlcRequest {
    client: Client,
    symbol: String,
    interval: Interval,
    price_type: PriceType,
    /// Unix timestamp in seconds. Returns up to 5000 candles from the specified
    /// value. Returns most recent candle if not included
    from: Option<i64>,
    /// Unix timestamp in seconds. Requires `from` argument.
    to: Option<i64>,
}

impl GetOhlcRequest {
    pub fn from(self, from: i64) -> Self {
        Self {
            from: Some(from),
            ..self
        }
    }

    pub fn to(self, to: i64) -> Self {
        Self {
            to: Some(to),
            ..self
        }
    }

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let mut url = format!(
            "/api/charts/v1/{}/{}/{}",
            self.price_type, self.symbol, self.interval
        );

        if let Some(from) = self.from {
            url.push_str(&format!("?from={}", from));

            if let Some(to) = self.to {
                url.push_str(&format!("?to={}", to));
            }
        }

        self.client.send_public(&url).await
    }

    pub async fn send(self) -> Result<GetOhlcResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Candle {
    pub time: i64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GetOhlcResponse {
    pub candles: Vec<Candle>,
}

impl Client {
    pub fn get_ohlc(
        &self,
        symbol: impl Into<String>,
        interval: Interval,
        price_type: PriceType,
    ) -> GetOhlcRequest {
        GetOhlcRequest {
            client: self.clone(),
            symbol: symbol.into(),
            interval,
            price_type,
            from: None,
            to: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Interval, PriceType};
    use crate::Client;

    #[test]
    fn get_ohlc_fetches_candles() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let client = Client::default();

            let from = chrono::Local::now();
            let from = from - chrono::Duration::hours(1);

            let res = client
                .get_ohlc("PI_XBTUSD", Interval::Min1, PriceType::Mark)
                .from(from.timestamp())
                .send()
                .await;

            dbg!(&res);
        });
    }
}
