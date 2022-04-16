# Kraken Futures REST API

A strongly-typed Rust SDK for the [Kraken Futures REST API](https://support.kraken.com/hc/en-us/sections/360012894412-Futures-API).

This crate is an *unofficial*, community-driven effort.

## Installation

```toml
[dependencies]
kraken_sdk_futures_rest = "0.2"
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

## Contact

For questions, suggestions, etc, you can reach the maintainer on [Twitter](https://twitter.com/gmosx).

## License

The software is distributed under the terms of both the MIT license and the Apache License (Version 2.0). See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Disclaimer

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.

## Copyright

Copyright © 2022 [George Moschovitis](https://gmosx.ninja).