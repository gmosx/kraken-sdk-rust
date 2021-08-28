# Kraken SDK component for the Kraken REST API

A strongly-typed Rust SDK for the [Kraken REST API](https://docs.kraken.com/rest/).

## Installation

```toml
[dependencies]
kraken_sdk_rest = "0.16"
```

## Usage

```rust
use kraken_sdk_rest::{Client, PairName, OrderSide};

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

Copyright © 2021 [George Moschovitis](https://gmosx.ninja).
