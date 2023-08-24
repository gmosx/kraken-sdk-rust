use kraken_ws_client::{api::SubscribeTickerRequest, client::DEFAULT_WS_URL, Client};

#[tokio::main]
async fn main() {
    let mut client = Client::connect(DEFAULT_WS_URL, None)
        .await
        .expect("cannot connect");

    let req = SubscribeTickerRequest::new(&["BTC/USD"]);

    client.send(req).await.expect("cannot send request");

    let mut messages = client.broadcast.subscribe();

    while let Ok(msg) = messages.recv().await {
        dbg!(&msg);
    }
}
