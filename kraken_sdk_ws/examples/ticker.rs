use futures::StreamExt;
use kraken_sdk_ws::{api::SubscribeTickerRequest, client::WS_URL, Client};

#[tokio::main]
async fn main() {
    let mut client = Client::connect(WS_URL, None).await.expect("cannot connect");

    let req = SubscribeTickerRequest::new(&["BTC/USD"]);

    client.send(req).await.expect("cannot send request");

    loop {
        if let Some(msg) = client.messages.next().await {
            dbg!(&msg);
        }
    }
}
