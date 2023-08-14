# Support for Kraken WebSocket API

# Kraken SDK component for the Kraken WebSocket API

A strongly-typed Rust SDK for the [Kraken WebSocket API](https://docs.kraken.com/websockets-v2).

This crate is an _unofficial_, community-driven effort.

## Installation

```toml
[dependencies]
kraken_sdk_ws = "0.18"
```

## Usage

```rs
use futures::StreamExt;
use kraken_client_ws::{api::SubscribeTickerRequest, client::WS_URL, Client};

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