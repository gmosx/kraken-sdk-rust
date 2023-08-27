use futures::StreamExt;
use kraken_ws_client::{api::SubscribeBookRequest, client::DEFAULT_WS_URL, types::Depth, Client};

#[tokio::main]
async fn main() {
    let mut client = Client::connect(DEFAULT_WS_URL, None)
        .await
        .expect("cannot connect");

    client
        .send(SubscribeBookRequest::symbol("BTC/USD").depth(Depth::D10))
        .await
        .expect("cannot send request");

    while let Some(event) = client.book_delta_events().next().await {
        dbg!(&event);
    }
}
