# Kraken CLI

A CLI for the Kraken Exchange.

## Setup

To install the executable from source, run:

```sh
cargo install --path .
```

## Usage

```sh
kraken --help

kraken market ticker BTC/USD
```

Additional functionality that will be supported _in the future_:

```sh
kraken balances
kraken balances --json --pretty
kraken balances -jp

kraken orders list-open
kraken orders list-open --json --pretty
kraken orders list-open -jp

kraken orders remove
kraken orders create
```

The `RUST_LOG` env variable is used to configure tracing, e.g.

```sh
RUST_LOG=debug kraken market ticker BTC/USD
```

## Status

This work is under active development and the API is expected to change.

## Contributing

Pull requests, issues and comments are welcome! Make sure to add tests for new
features and bug fixes.

## License

This work is licensed under the Apache-2.0 License. See
[LICENSE.txt](LICENSE.txt) or <https://spdx.org/licenses/Apache-2.0.html> for
details.

## Copyright

Copyright © 2023 [Georgios Moschovitis](https://gmosx.ninja).
