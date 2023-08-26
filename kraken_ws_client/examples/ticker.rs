use futures::StreamExt;
use kraken_ws_client::{client::DEFAULT_WS_URL, Client};

#[tokio::main]
async fn main() {
    let mut client = Client::connect(DEFAULT_WS_URL, None)
        .await
        .expect("cannot connect");

    client.subscribe_ticker("BTC/USD").await;

    while let Some(event) = client.ticker_events().next().await {
        dbg!(&event);
    }
}
