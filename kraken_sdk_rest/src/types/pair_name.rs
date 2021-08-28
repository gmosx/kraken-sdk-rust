use crate::has_custom_code;

use super::AssetName;
use std::fmt::Display;

/// Convenience struct to build a KrakenAPI-compatible pair.
pub struct PairName {
    base: AssetName,
    quote: AssetName,
}

impl PairName {
    pub fn from(base: &str, quote: &str) -> Self {
        if has_custom_code(base) {
            return Self {
                base: AssetName::from(base),
                quote: AssetName::from(quote),
            };
        }
        Self {
            base: AssetName {
                name: base.to_string(),
            },
            quote: AssetName {
                name: quote.to_string(),
            },
        }
    }
}

impl Display for PairName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.base, self.quote)
    }
}

impl From<PairName> for String {
    fn from(pair: PairName) -> Self {
        pair.to_string()
    }
}

impl From<&PairName> for String {
    fn from(pair: &PairName) -> Self {
        pair.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_name() {
        let pair = PairName::from("XBT", "USD");
        assert_eq!(pair.to_string(), "XXBTZUSD");

        let pair = PairName::from("BTC", "USD");
        assert_eq!(pair.to_string(), "XXBTZUSD");

        let pair = PairName::from("XRP", "USD");
        assert_eq!(pair.to_string(), "XXRPZUSD");

        // https://api.kraken.com/0/public/Ticker?pair=DOTUSD
        let pair = PairName::from("DOT", "USD");
        assert_eq!(pair.to_string(), "DOTUSD");

        // https://api.kraken.com/0/public/Ticker?pair=KSMUSD
        let pair = PairName::from("KSM", "USD");
        assert_eq!(pair.to_string(), "KSMUSD");
    }
}
