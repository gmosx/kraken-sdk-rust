use futures::StreamExt;
use kraken_ws_client::{client::DEFAULT_WS_URL, types::BookDepth, Client};

#[tokio::main]
async fn main() {
    let mut client = Client::connect(DEFAULT_WS_URL, None)
        .await
        .expect("cannot connect");

    client.subscribe_book("BTC/USD", BookDepth::D10).await;

    while let Some(event) = client.book_delta_events().next().await {
        dbg!(&event);
    }
}
