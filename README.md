# Kraken REST API Client

A strongly-typed Rust client for the [Kraken REST API](https://www.kraken.com/features/api).

## Installation

```
[dependencies]
kraken_client = "0.4.0"
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

This library is under development and the API is expected to change.

**It's not ready for production use**.

## License

This library is licensed under either of Apache License, Version 2.0 or MIT license, at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Copyright

Copyright (c) 2021 George Moschovitis.
