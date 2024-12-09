use crate::{
    types::{OrderDescription, OrderSide, OrderType},
    Client, Result,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// - <https://www.kraken.com/features/api#add-standard-order>
/// - <https://api.kraken.com/0/private/AddOrder>
/// - <https://support.kraken.com/hc/en-us/articles/205893708-Minimum-order-size-volume-for-trading>
#[must_use = "Does nothing until you send or execute it"]
pub struct AddOrderRequest {
    client: Client,
    pair: String,
    order_side: OrderSide,
    order_type: OrderType,
    price: Option<String>,
    /// Secondary price.
    price2: Option<String>,
    /// Order volume in lots.
    volume: String,
    // Amount of leverage desired.
    leverage: Option<String>,
    /// Comma delimited list of order flags:
    /// - fcib = prefer fee in base currency
    /// - fciq = prefer fee in quote currency
    /// - nompp = no market price protection
    /// - post = post only order (available when ordertype = limit)
    oflags: Option<String>,
    /// Time-in-force of the order to specify how long it should remain in the
    /// order book before being cancelled.
    timeinforce: Option<String>,
    /// Scheduled start time.
    starttm: Option<String>,
    /// Expiration time.
    expiretm: Option<String>,
    /// User reference id.
    userref: Option<i32>,
    /// Validate inputs only, do not submit order.
    validate: Option<bool>,
    close_order_type: Option<OrderType>,
    close_price: Option<String>,
    close_price2: Option<String>,
}

impl AddOrderRequest {
    pub fn price(self, price: &str) -> Self {
        Self {
            price: Some(price.into()),
            ..self
        }
    }

    pub fn price2(self, price: &str) -> Self {
        Self {
            price2: Some(price.into()),
            ..self
        }
    }

    // TODO: add typed flags builder.

    /// oflags = comma delimited list of order flags:
    /// fcib = prefer fee in base currency
    /// fciq = prefer fee in quote currency
    /// nompp = no market price protection
    /// post = post only order (available when ordertype = limit)
    pub fn flags(self, flags: &str) -> Self {
        Self {
            oflags: Some(flags.to_string()),
            ..self
        }
    }

    /// A post only order prohibits a limit order to get immediately filled
    /// 'at market' and incur (potentially increased) market-order fees.
    pub fn post_only(self) -> Self {
        // TODO: check that this is a limit order.
        // TODO: don't override existing flags.
        Self {
            oflags: Some("post".to_string()),
            ..self
        }
    }

    /// Conditionally set `post_only`
    pub fn post(self, post: bool) -> Self {
        if post {
            self.post_only()
        } else {
            self
        }
    }

    // #todo introduce a timeinforce enum!
    /// Time-in-force of the order to specify how long it should remain in the
    /// order book before being cancelled. GTC (Good-'til-cancelled) is default
    /// if the parameter is omitted. IOC (immediate-or-cancel) will immediately
    /// execute the amount possible and cancel any remaining balance rather than
    /// resting in the book. GTD (good-'til-date), if specified, must coincide
    /// with a desired expiretm.
    pub fn timeinforce(self, timeinforce: &str) -> Self {
        Self {
            timeinforce: Some(timeinforce.to_owned()),
            ..self
        }
    }

    /// Start time
    /// +<n> = expire <n> seconds from now
    /// <n> = unix timestamp of expiration time
    pub fn starttm(self, starttm: &str) -> Self {
        Self {
            starttm: Some(urlencoding::encode(starttm).to_string()),
            ..self
        }
    }

    pub fn start_after(self, seconds: u32) -> Self {
        self.starttm(&format!("+{}", seconds))
    }

    // TODO:
    // Add convenience functions
    // - `expire_timestamp`

    /// Expiration time
    /// +<n> = expire <n> seconds from now
    /// <n> = unix timestamp of expiration time
    pub fn expiretm(self, expiretm: &str) -> Self {
        Self {
            expiretm: Some(urlencoding::encode(expiretm).to_string()),
            ..self
        }
    }

    pub fn expire_after(self, seconds: u32) -> Self {
        self.expiretm(&format!("+{}", seconds))
    }

    pub fn userref(self, userref: i32) -> Self {
        Self {
            userref: Some(userref),
            ..self
        }
    }

    pub fn close_order(
        self,
        close_order_type: OrderType,
        close_price: Option<String>,
        close_price2: Option<String>,
    ) -> Self {
        Self {
            close_order_type: Some(close_order_type),
            close_price,
            close_price2,
            ..self
        }
    }

    pub fn close_limit_order(self, close_price: &str) -> Self {
        Self {
            close_order_type: Some(OrderType::Limit),
            close_price: Some(close_price.to_string()),
            ..self
        }
    }

    pub fn validate(self, validate: bool) -> Self {
        Self {
            validate: Some(validate),
            ..self
        }
    }

    pub fn validate_only(self) -> Self {
        Self {
            validate: Some(true),
            ..self
        }
    }

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let mut query = format!(
            "pair={}&type={}&ordertype={}&volume={}",
            self.pair, self.order_side, self.order_type, self.volume,
        );

        if let Some(price) = &self.price {
            query.push_str(&format!("&price={}", price));
        }

        if let Some(price2) = &self.price2 {
            query.push_str(&format!("&price2={}", price2));
        }

        if let Some(leverage) = &self.leverage {
            query.push_str(&format!("&leverage={}", leverage));
        }

        if let Some(oflags) = &self.oflags {
            query.push_str(&format!("&oflags={}", oflags));
        }

        if let Some(timeinforce) = &self.timeinforce {
            query.push_str(&format!("&timeinforce={}", timeinforce));
        }

        if let Some(starttm) = &self.starttm {
            query.push_str(&format!("&starttm={}", starttm));
        }

        if let Some(expiretm) = &self.expiretm {
            query.push_str(&format!("&expiretm={}", expiretm));
        }

        if let Some(close_order_type) = &self.close_order_type {
            query.push_str(&format!("&close[ordertype]={}", close_order_type));

            if let Some(close_price) = &self.close_price {
                query.push_str(&format!("&close[price]={}", close_price));
            }

            if let Some(close_price2) = &self.close_price2 {
                query.push_str(&format!("&close[price2]={}", close_price2));
            }
        }

        if let Some(userref) = &self.userref {
            query.push_str(&format!("&userref={}", userref));
        }

        if let Some(true) = &self.validate {
            query.push_str("&validate=true");
        }

        self.client
            .send_private("/0/private/AddOrder", Some(query))
            .await
    }

    pub async fn send(self) -> Result<AddOrderResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize)]
pub struct AddOrderResponse {
    pub descr: OrderDescription,
    /// Transaction ids (if order was added successfully)
    pub txid: Option<Vec<String>>,
}

impl Client {
    pub fn add_order(
        &self,
        pair: &str,
        order_side: OrderSide,
        order_type: OrderType,
        volume: &str,
    ) -> AddOrderRequest {
        AddOrderRequest {
            client: self.clone(),
            pair: pair.to_string(),
            order_side,
            order_type,
            price: None,
            price2: None,
            volume: volume.to_string(),
            leverage: None,
            oflags: None,
            timeinforce: None,
            starttm: None,
            expiretm: None,
            userref: None,
            validate: None,
            close_order_type: None,
            close_price: None,
            close_price2: None,
        }
    }

    pub fn add_market_order(
        &self,
        pair: &str,
        order_side: OrderSide,
        volume: &str,
    ) -> AddOrderRequest {
        AddOrderRequest {
            client: self.clone(),
            pair: pair.to_string(),
            order_side,
            order_type: OrderType::Market,
            price: None,
            price2: None,
            volume: volume.to_string(),
            leverage: None,
            oflags: None,
            timeinforce: None,
            starttm: None,
            expiretm: None,
            userref: None,
            validate: None,
            close_order_type: None,
            close_price: None,
            close_price2: None,
        }
    }

    pub fn add_limit_order(
        &self,
        pair: &str,
        order_side: OrderSide,
        volume: &str,
        price: &str,
    ) -> AddOrderRequest {
        AddOrderRequest {
            client: self.clone(),
            pair: pair.to_string(),
            order_side,
            order_type: OrderType::Limit,
            price: Some(price.to_string()),
            price2: None,
            volume: volume.to_string(),
            leverage: None,
            oflags: None,
            timeinforce: None,
            starttm: None,
            expiretm: None,
            userref: None,
            validate: None,
            close_order_type: None,
            close_price: None,
            close_price2: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, OrderSide};

    #[tokio::test]
    async fn test_post_only() {
        let client = Client::default();

        let req = client
            .add_market_order("XXBTZUSD", OrderSide::Buy, "0.1")
            .validate_only()
            .post_only();

        assert_eq!(req.oflags, Some("post".to_string()));
    }
}
