use serde::Serialize;

#[derive(Serialize)]
pub struct Subscription<'a> {
    pub name: &'a str,
}

#[derive(Serialize)]
pub struct SubscriptionRequest<'a> {
    pub event: &'a str,
    pub pair: &'a [&'a str],
    pub subscription: Subscription<'a>,
}

impl SubscriptionRequest<'_> {
    pub fn new<'a>(name: &'a str, pair: &'a [&'a str]) -> SubscriptionRequest<'a> {
        SubscriptionRequest {
            event: "subscribe",
            pair,
            subscription: Subscription { name },
        }
    }
}
