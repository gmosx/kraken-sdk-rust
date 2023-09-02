use futures::StreamExt;
use kraken_ws_client::api::SubscribeTickerRequest;

#[tokio::main]
async fn main() {
    let mut client = kraken_ws_client::connect_public()
        .await
        .expect("cannot connect");

    client
        .send(SubscribeTickerRequest::symbol("BTC/USD"))
        .await
        .expect("cannot send request");

    while let Some(event) = client.ticker_events().next().await {
        dbg!(&event);
    }
}
