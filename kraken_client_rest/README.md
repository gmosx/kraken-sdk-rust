# Kraken REST API

A strongly-typed Rust SDK for the [Kraken REST API](https://docs.kraken.com/rest/).

This crate is an *unofficial*, community-driven effort.

## Installation

```toml
[dependencies]
kraken_client_rest = "0.20"
```

## Usage

```rust
use kraken_client_rest::{Client, PairName, OrderSide};

let client = Client::new(
    "YOUR-API-KEY",
    "YOUR_API-SECRET",
);

let resp = client.get_server_time().send().await?;

println!("{}", resp.unixtime);

let pair = PairName::from("BTC", "USD");
let req = client.get_ohlc_data(&pair).interval(Interval::Day1);
let resp = req.send().await;

println!("{:?}", resp);

let pair = "XXRPZUSD";
let resp = client
    .add_limit_order(pair, OrderSide::Buy, "20", "0.10")
    .expire_after(60 * 60)
    .userref(123)
    .validate_only()
    .send()
    .await?;

println!("{:?}", resp);

let resp = client.cancel_order("O6CIT1-NABRS-TMVZ1X").send().await?;

println!("{}", resp.count);
```

## FAQ

### Why provide both execute and send methods for API endpoint handlers?

Providing the lower-level `execute` method allows for more flexibility. Since `execute` is generic you can pass any type of object to deserialize the response to, e.g. you could deserialize to a `HashMap` instead of the 'default' response for each API call. Or you could use a custom struct with only the fields you are interested in.

## Status

The software is under active development and the API is expected to change.

## Contributing

Pull requests, issues and comments are welcome! Make sure to add tests for new features and bug fixes.

## License

This work is licensed under the Apache-2.0 License. See [LICENSE.txt](LICENSE.txt) or <https://spdx.org/licenses/Apache-2.0.html> for details.

## Copyright

Copyright © 2022 [Georgios Moschovitis](https://gmosx.ninja).