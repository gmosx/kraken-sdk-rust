use std::{env, fmt::Debug};

use clap::{Arg, ArgAction, ArgMatches, Command};
use kraken_rest_client::Client;
use serde::Serialize;
// #todo What is a good name?
pub fn add_json_args(command: Command) -> Command {
    command
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .help("Format response as JSON")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("pretty")
                .short('p')
                .long("pretty")
                .help("Pretty-print the response")
                .action(ArgAction::SetTrue),
        )
}

// #todo Find better name.
pub fn format_response<R>(resp: R, matches: &ArgMatches) -> String
where
    R: Serialize + Debug,
{
    if matches.get_flag("json") {
        if matches.get_flag("pretty") {
            serde_json::to_string_pretty(&resp).unwrap().to_string()
        } else {
            serde_json::to_string(&resp).unwrap().to_string()
        }
    } else {
        format!("{:?}", resp)
    }
}

// #todo find a better name.
pub fn make_private_client() -> Client {
    let api_key = env::var("KRAKEN_API_KEY").expect("KRAKEN_API_KEY must be set");
    let api_secret = env::var("KRAKEN_API_SECRET").expect("KRAKEN_API_SECRET must be set");

    Client::new(api_key, api_secret)
}
