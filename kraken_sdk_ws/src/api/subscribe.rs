use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SubscriptionName {
    Book,
    Ohlc,
    OpenOrders,
    OwnTrades,
    Spread,
    Ticker,
    Trade,
    All,
}

#[derive(Serialize)]
pub struct Subscription {
    pub name: SubscriptionName,
}

#[derive(Serialize)]
pub struct SubscriptionRequest<'a> {
    pub event: &'a str,
    pub pair: &'a [&'a str],
    pub subscription: Subscription,
}

impl SubscriptionRequest<'_> {
    pub fn new<'a>(name: SubscriptionName, pair: &'a [&'a str]) -> SubscriptionRequest<'a> {
        SubscriptionRequest {
            event: "subscribe",
            pair,
            subscription: Subscription { name },
        }
    }

    pub fn ticker<'a>(pair: &'a [&'a str]) -> SubscriptionRequest<'a> {
        Self::new(SubscriptionName::Ticker, pair)
    }
}
