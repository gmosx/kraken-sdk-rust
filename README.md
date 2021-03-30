# Kraken REST API Client

A strongly-typed Rust client for the [Kraken REST API](https://www.kraken.com/features/api).

## Installation

```
[dependencies]
kraken_client = "0.7.1"
```

## Usage

```rust
let client = Client::new(
    "YOUR-API-KEY",
    "YOUR_API-SECRET",
);

let resp = client.get_server_time().send().await?;

println!("{}", resp.unixtime);

let pair = "XXRPZUSD";
let resp = client
    .add_limit_order(pair, OrderSide::Buy, "20", "0.10")
    .userref(123)
    .validate_only()
    .send()
    .await?;

println!("{:?}", resp);

let resp = client.cancel_order("O6CIT1-NABRS-TMVZ1X").send().await?;

println!("{}", resp.count);
```

## Status

The software is under active development and the API is expected to change.

**It's not ready for production use**.

## Links

- https://support.kraken.com/hc/en-us/articles/360000678446-Cryptocurrencies-available-on-Kraken
- https://support.kraken.com/hc/en-us/articles/201893658-Currency-pairs-available-for-trading-on-Kraken

## Contributing

Pull requests, issues and comments are welcome! Make sure to add tests for new features and bug fixes.

## Contact

For questions, suggestions, etc, you can reach the maintainer on [Twitter](https://twitter.com/gmosx).

## License

The software is distributed under the terms of both the MIT license and the Apache License (Version 2.0). See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Copyright

Copyright (c) 2021 [George Moschovitis](https://gmosx.ninja).
