pub mod api;
pub mod client;
pub mod error;
mod serde;
mod sign;
pub mod types;

pub use api::get_ohlc_data::Interval;
pub use client::{Client, Result};
pub use types::*;

pub const MAX_OPEN_ORDERS_COUNT_STARTER: usize = 60;
pub const MAX_OPEN_ORDERS_COUNT_IMMEDIATE: usize = 80;
pub const MAX_OPEN_ORDERS_COUNT_PRO: usize = 225;

pub const MAX_SCHEDULED_ORDERS_COUNT_STARTER: usize = 15;
pub const MAX_SCHEDULED_ORDERS_COUNT_IMMEDIATE: usize = 25;
pub const MAX_SCHEDULED_ORDERS_COUNT_PRO: usize = 40;
