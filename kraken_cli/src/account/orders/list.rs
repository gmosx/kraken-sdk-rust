use clap::ArgMatches;

use crate::util::{format_response, make_private_client};

pub async fn account_orders_list(matches: &ArgMatches) -> anyhow::Result<()> {
    let client = make_private_client();

    let resp = client.get_open_orders().send().await?;

    println!("{}", format_response(resp.open, matches));

    Ok(())
}
