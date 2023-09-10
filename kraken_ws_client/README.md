# Kraken WebSocket API Client

A strongly-typed Rust SDK for the [Kraken WebSocket API](https://docs.kraken.com/websockets-v2).

This crate is an _unofficial_, community-driven effort.

## Installation

```toml
[dependencies]
kraken_ws_client = "0.24"
```

## Usage

```rs
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
```

```rs
use kraken_rest_client::Client as RestClient;

let api_key = std::env::var("KRAKEN_API_KEY").expect("api key not defined");
let api_secret = std::env::var("KRAKEN_API_SECRET").expect("api secret not defined");

let rest_client = RestClient::new(api_key, api_secret);
let resp = rest_client.get_web_sockets_token().send().await?;
let token = resp.token;

let mut ws_private_client = kraken_ws_client::connect_private(token)
    .await
    .expect("cannot connect");

ws_private_client
    .send(SubscribeExecutionsRequest::new())
    .await
    .expect("cannot send request");

while let Ok(msg) = ws_private_client.messages().recv().await {
    dbg!(msg);
}
```

or run the example:

```rs
cargo run --example ticker
```

## Status

The software is under active development and the API is expected to change.

## Contributing

Pull requests, issues and comments are welcome! Make sure to add tests for new features and bug fixes.

## License

This work is licensed under the Apache-2.0 License. See [LICENSE.txt](LICENSE.txt) or <https://spdx.org/licenses/Apache-2.0.html> for details.

## Copyright

Copyright © 2022 [Georgios Moschovitis](https://gmosx.ninja).