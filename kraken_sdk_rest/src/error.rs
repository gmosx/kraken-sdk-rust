use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error {
    #[error("internal error: {0}")]
    Internal(String),
    #[error("failed request: {err}")]
    FailedRequest { err: String, status: Option<u16> },
    #[error("not authorized: missing api_credentials")]
    Unauthorized,
    #[error("api error: {0}")]
    Api(String),
}

impl Error {
    pub(crate) fn internal(message: impl fmt::Display) -> Self {
        Self::Internal(message.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::FailedRequest {
            err: e.to_string(),
            status: e.status().map(|c| c.as_u16()),
        }
    }
}

impl From<hmac::crypto_mac::InvalidKeyLength> for Error {
    fn from(e: hmac::crypto_mac::InvalidKeyLength) -> Self {
        Self::Internal(e.to_string())
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(e: std::time::SystemTimeError) -> Self {
        Self::Internal(e.to_string())
    }
}
