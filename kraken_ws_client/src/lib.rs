//! A strongly-typed client for Kraken.
//! https://docs.kraken.com/websockets-v2

pub mod api;
pub mod client;
pub mod error;
pub mod types;

mod util;

pub use client::{PrivateClient, PublicClient};
pub use error::Error;
pub use util::Result;

pub async fn connect_public() -> Result<PublicClient> {
    PublicClient::connect().await
}

pub async fn connect_private(token: impl Into<String>) -> Result<PrivateClient> {
    PrivateClient::connect(token).await
}
