use clap::{Arg, Command};
use kraken_cli::{
    account::balance::account_balance,
    market::{ohlc::market_ohlc, ticker::market_ticker},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // The `market` subcommand.

    let market_cmd = Command::new("market")
        .about("Market")
        .subcommand(
            Command::new("ohlc").about("Fetch OHLC data").arg(
                // Positional argument!
                Arg::new("PAIR")
                    .help("Selects the market to fetch")
                    .required(false)
                    .index(1),
            ),
        )
        .subcommand(
            Command::new("ticker").about("Fetch Ticker data").arg(
                // Positional argument!
                Arg::new("PAIR")
                    .help("Selects the market to fetch")
                    .required(false)
                    .index(1),
            ),
        );

    // The `account` subcommand.

    let account_cmd = Command::new("account")
        .about("Account data")
        .subcommand(Command::new("balance").about("List account balances"));

    // The program command.

    let mut kraken_cmd = Command::new("kraken")
        .author("Georgios Moschovitis, george.moschovitis@gmail.com")
        .version(VERSION)
        .about("A CLI for the Kraken Exchange")
        .after_help("The kraken CLI provides access to the Kraken exchange.")
        .subcommand(market_cmd)
        .subcommand(account_cmd);

    let matches = kraken_cmd.get_matches_mut();

    if let Some(market_matches) = matches.subcommand_matches("market") {
        if let Some(matches) = market_matches.subcommand_matches("ohlc") {
            market_ohlc(matches).await?;
        } else if let Some(matches) = market_matches.subcommand_matches("ticker") {
            market_ticker(matches).await?;
        }
        // #todo Have a default sub-command?
        // #todo Else?
    } else if let Some(account_matches) = matches.subcommand_matches("account") {
        if let Some(matches) = account_matches.subcommand_matches("balance") {
            account_balance(matches).await?;
        } else {
            // The default subcommans is `list`.
            account_balance(account_matches).await?;
        }
    } else {
        kraken_cmd.print_long_help().unwrap();
    }

    Ok(())
}
