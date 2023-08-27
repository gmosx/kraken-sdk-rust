use futures::StreamExt;
use kraken_ws_client::{api::SubscribeTickerRequest, client::DEFAULT_WS_URL, Client};

#[tokio::main]
async fn main() {
    let mut client = Client::connect(DEFAULT_WS_URL, None)
        .await
        .expect("cannot connect");

    client
        .send_public(SubscribeTickerRequest::symbol("BTC/USD"))
        .await
        .expect("cannot send request");

    while let Some(event) = client.ticker_events().next().await {
        dbg!(&event);
    }
}
