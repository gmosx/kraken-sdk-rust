use clap::{Arg, Command};
use kraken_cli::market::ohlc::market_ohlc;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // `market` subcommand.

    let market_cmd = Command::new("market").about("Market").subcommand(
        Command::new("ohlc").about("Fetch OHLC data").arg(
            // Positional argument!
            Arg::new("MARKET_NAME")
                .help("Selects the market to fetch")
                .required(false)
                .index(1),
        ),
    );

    let mut kraken_cmd = Command::new("kraken")
        .author("Georgios Moschovitis, george.moschovitis@gmail.com")
        .version(VERSION)
        .about("A CLI for the Kraken Exchange")
        .after_help("The kraken CLI provides access to the Kraken exchange.")
        .subcommand(market_cmd);

    let matches = kraken_cmd.get_matches_mut();

    if let Some(market_matches) = matches.subcommand_matches("market") {
        if let Some(ohlc_matches) = market_matches.subcommand_matches("ohlc") {
            market_ohlc(ohlc_matches).await?;
        }
    } else {
        kraken_cmd.print_long_help().unwrap();
    }

    Ok(())
}
