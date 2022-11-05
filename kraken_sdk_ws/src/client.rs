use crate::{types::SubscriptionName, Socket};
use tokio_tungstenite::tungstenite::Message;

pub struct Client {
    pub socket: Socket,
}

// TODO: return stream.
// TODO: implement reference counting for subscriptions
// TODO: run concurrently in standalone thread.

fn name_from_subscription(name: SubscriptionName) -> String {
    match name {
        SubscriptionName::Book => "book",
        SubscriptionName::Ohlc => "ohlc",
        SubscriptionName::OpenOrders => "openOrders",
        SubscriptionName::OwnTrades => "ownTrades",
        SubscriptionName::Spread => "spread",
        SubscriptionName::Ticker => "ticker",
        SubscriptionName::Trade => "trade",
        SubscriptionName::All => "*",
    }
    .to_owned()
}

impl Client {
    pub async fn connect_public() -> Self {
        Self {
            socket: Socket::connect_public().await,
        }
    }

    pub async fn send(
        &mut self,
        msg: Message,
    ) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        self.socket.send(msg).await
    }

    pub async fn next(&mut self) -> Option<Result<Message, tokio_tungstenite::tungstenite::Error>> {
        self.socket.next().await
    }
}
