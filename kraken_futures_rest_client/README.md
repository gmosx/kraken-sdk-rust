# Kraken Futures REST API

A strongly-typed Rust SDK for the [Kraken Futures REST API](https://support.kraken.com/hc/en-us/sections/360012894412-Futures-API).

This crate is an *unofficial*, community-driven effort.

## Installation

```toml
[dependencies]
kraken_futures_client_rest = "0.2"
```

## Usage

```rust
let client = Client::default();

let symbol = "PI_XBTUSD";
let interval = Interval::Min1;
let now = chrono::Local::now();
let from = now - chrono::Duration::minutes(6_000);
let to = from + (1_000 * 60);

let res = client
    .get_ohlc(symbol, interval, price_type)
    .from(from)
    .to(i64::min(now, to))
    .send()
    .await?;

println!("{}", res.candles);
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