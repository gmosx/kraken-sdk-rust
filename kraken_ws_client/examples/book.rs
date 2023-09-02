use futures::StreamExt;
use kraken_ws_client::{api::SubscribeBookRequest, types::Depth};

#[tokio::main]
async fn main() {
    let mut client = kraken_ws_client::connect_public()
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
