use clap::ArgMatches;
use kraken_rest_client::Client;

pub async fn market_ticker(
    matches: &ArgMatches,
    // offers_matches: &ArgMatches,
) -> anyhow::Result<()> {
    let pair: &String = matches.get_one("PAIR").unwrap();

    let client = Client::default();

    let resp = client.get_tickers(pair).send().await;

    match resp {
        Ok(resp) => println!("{:?}", resp),
        Err(error) => eprintln!("{:?}", error),
    }

    Ok(())
}
