use clap::ArgMatches;

pub async fn market_ohlc(
    matches: &ArgMatches,
    // offers_matches: &ArgMatches,
) -> anyhow::Result<()> {
    let market_name: &String = matches.get_one("MARKET_NAME").unwrap();
    println!("**** OHLC *** {market_name}");
    Ok(())
}
