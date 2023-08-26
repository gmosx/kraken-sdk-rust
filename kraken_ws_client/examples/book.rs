use futures::StreamExt;
use kraken_ws_client::{client::DEFAULT_WS_URL, types::BookDepth, Client};

#[tokio::main]
async fn main() {
    let mut client = Client::connect(DEFAULT_WS_URL, None)
        .await
        .expect("cannot connect");

    client.subscribe_book("BTC/USD", BookDepth::D10).await;

    let mut book_events = client.book_events.unwrap();

    while let Some(event) = book_events.next().await {
        dbg!(&event);
    }
}
