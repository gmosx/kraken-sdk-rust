use clap::ArgMatches;
use kraken_rest_client::{OrderSide, OrderType};

use crate::util::{format_response, make_private_client};

// #todo Extract the helper methods to a `util` module.

// #todo consider the name `parse_order_side`.
pub fn order_side_from_string(string: &str) -> OrderSide {
    match string.to_lowercase().as_ref() {
        "buy" => OrderSide::Buy,
        "sell" => OrderSide::Sell,
        _ => panic!("invalid order side"),
    }
}

pub fn order_type_from_string(string: &str) -> OrderType {
    match string.to_lowercase().as_ref() {
        "market" => OrderType::Market,
        "limit" => OrderType::Limit,
        // #todo Add support for other order types.
        _ => panic!("invalid order side"),
    }
}

pub async fn account_orders_create(matches: &ArgMatches) -> anyhow::Result<()> {
    let client = make_private_client();

    let order_type: &String = matches
        .get_one("ORDER_TYPE")
        .expect("valid order type argument");

    let side: &String = matches.get_one("SIDE").expect("valid side argument");
    let pair: &String = matches.get_one("PAIR").expect("valid pair argument");
    let volume: &String = matches.get_one("VOLUME").expect("valid volume argument");
    let price: &String = matches.get_one("PRICE").expect("valid price argument");

    // println!("{order_type}");
    // println!("{side}");
    // println!("{pair}");
    // println!("{volume}");
    // println!("{price}");

    // #todo Add support for more order types.
    // #todo Add support for validate_only and post_only.

    let resp = client
        .add_order(
            pair,
            order_side_from_string(side),
            order_type_from_string(order_type),
            volume,
        )
        .price(price)
        // .validate_only()
        .post_only()
        .send()
        .await?;

    println!("{}", format_response(resp.descr, matches));

    Ok(())
}
