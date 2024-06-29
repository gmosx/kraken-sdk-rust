use clap::ArgMatches;

pub async fn market_ohlc(
    matches: &ArgMatches,
    // offers_matches: &ArgMatches,
) -> anyhow::Result<()> {
    let pair: &String = matches.get_one("PAIR").unwrap();
    println!("**** OHLC *** {pair}");
    Ok(())
}
