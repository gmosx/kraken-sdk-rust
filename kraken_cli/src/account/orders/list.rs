use std::env;

use clap::ArgMatches;
use kraken_rest_client::Client;

use crate::util::format_response;

pub async fn account_orders_list(matches: &ArgMatches) -> anyhow::Result<()> {
    let api_key = env::var("KRAKEN_API_KEY").expect("KRAKEN_API_KEY must be set");
    let api_secret = env::var("KRAKEN_API_SECRET").expect("KRAKEN_API_SECRET must be set");

    let client = Client::new(api_key, api_secret);

    let resp = client.get_open_orders().send().await?;

    println!("{}", format_response(resp.open, matches));

    Ok(())
}
