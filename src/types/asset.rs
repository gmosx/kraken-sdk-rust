use std::fmt::Display;

pub fn asset_name_to_code(name: &str) -> String {
    let code = match name {
        "BTC" | "XBT" => "XXBT",
        "XRP" => "XXRP",
        "XLM" => "XXLM",
        "LTC" => "XLTC",
        "XDG" => "XXDG",
        "ETH" => "XETH",
        "ETC" => "XETC",
        "REP" => "XREP",
        "ZEC" => "XZEC",
        "XMR" => "XXMR",
        "MLN" => "XMLN",

        "USD" => "ZUSD",
        "EUR" => "ZEUR",
        "GBP" => "ZGBP",
        "CAD" => "ZCAD",
        "AUD" => "ZAUD",
        "JPY" => "ZJPY",

        _ => name,
    };

    String::from(code)
}

pub struct Asset {
    pub name: String,
}

impl Display for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Asset {
    pub fn from(name: &str) -> Self {
        Self {
            name: asset_name_to_code(name),
        }
    }
}

impl Into<Asset> for &str {
    fn into(self) -> Asset {
        Asset::from(self)
    }
}

impl Into<Asset> for String {
    fn into(self) -> Asset {
        Asset::from(&self)
    }
}

impl From<Asset> for String {
    fn from(asset: Asset) -> Self {
        asset.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_name() {
        let asset = Asset::from("BTC");
        assert_eq!(asset.to_string(), "XXBT");

        let asset = Asset::from("XBT");
        assert_eq!(asset.to_string(), "XXBT");

        let asset = Asset::from("ETH");
        assert_eq!(asset.to_string(), "XETH");

        let asset: Asset = "USD".into();
        assert_eq!(asset.to_string(), "ZUSD");

        let a: String = asset.into();
        assert_eq!(a, "ZUSD");
    }
}
