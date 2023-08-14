//! A strongly-typed client for Kraken.
//! https://docs.kraken.com/websockets-v2

pub mod api;
pub mod client;
pub mod error;
pub mod types;

mod util;

// #[cfg(test)]
// mod client_tests;

pub use client::Client;
pub use util::Result;
