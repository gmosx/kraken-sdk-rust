use futures::StreamExt;
use kraken_ws_client::{client::DEFAULT_WS_URL, Client};

#[tokio::main]
async fn main() {
    let mut client = Client::connect(DEFAULT_WS_URL, None)
        .await
        .expect("cannot connect");

    client.subscribe_ticker("BTC/USD").await;

    let mut ticker_events = client.ticker_events.unwrap();

    while let Some(event) = ticker_events.next().await {
        dbg!(&event);
    }
}
