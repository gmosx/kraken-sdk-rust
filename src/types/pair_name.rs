use super::AssetName;
use std::fmt::Display;

pub struct PairName {
    base: AssetName,
    quote: AssetName,
}

impl PairName {
    pub fn from(base: &str, quote: &str) -> Self {
        Self {
            base: AssetName::from(base),
            quote: AssetName::from(quote),
        }
    }
}

impl Display for PairName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.base, self.quote)
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
    }
}
