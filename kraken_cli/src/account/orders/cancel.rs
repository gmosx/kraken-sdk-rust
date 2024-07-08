use clap::ArgMatches;

use crate::util::{format_response, make_private_client};

pub async fn account_orders_cancel(matches: &ArgMatches) -> anyhow::Result<()> {
    let client = make_private_client();

    let txid: &String = matches
        .get_one("ORDER_TXID")
        .expect("valid order txid argument");

    let resp = client.cancel_order(txid).send().await?;

    println!("{}", format_response(resp.count, matches));

    Ok(())
}
