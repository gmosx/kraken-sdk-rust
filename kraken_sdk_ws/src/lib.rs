//! A strongly-typed client for Kraken.

pub mod api;
pub mod client;
pub mod error;

mod util;

// #[cfg(test)]
// mod client_tests;

pub use client::Client;
pub use util::Result;
