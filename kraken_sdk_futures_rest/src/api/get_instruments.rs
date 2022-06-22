use crate::{client::Client, Result};
use serde::{de::DeserializeOwned, Deserialize};

/// This endpoint returns specifications for all currently listed Futures
/// contracts and indices.
///
/// ## Documentation
///
/// - https://support.kraken.com/hc/en-us/articles/360022635672-Instruments
///
/// ## Sample call
///
/// - /api/v3/instruments
#[must_use = "Does nothing until you send or execute it"]
pub struct GetInstrumentsRequest {
    client: Client,
}

impl GetInstrumentsRequest {
    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        self.client
            .send_public("/derivatives/api/v3/instruments")
            .await
    }

    pub async fn send(self) -> Result<GetInstrumentsResponse> {
        self.execute().await
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct MarginLevel {
    pub contracts: Option<u64>,
    #[serde(rename = "initialMargin")]
    pub initial_margin: f64,
    #[serde(rename = "maintenanceMargin")]
    pub maintenance_margin: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Instrument {
    pub symbol: String,
    #[serde(rename = "type")]
    pub instrument_type: String,
    pub underlying: Option<String>,
    #[serde(rename = "tickSize")]
    pub tick_size: Option<f64>,
    #[serde(rename = "contractSize")]
    pub contract_size: Option<f64>,
    pub tradeable: bool,
    #[serde(rename = "marginLevels")]
    pub margin_levels: Option<Vec<MarginLevel>>,
    // TODO: add more fields.
}

#[derive(Debug, Deserialize, Clone)]
pub struct GetInstrumentsResponse {
    pub result: String,
    pub instruments: Vec<Instrument>,
}

impl Client {
    pub fn get_instruments(&self) -> GetInstrumentsRequest {
        GetInstrumentsRequest {
            client: self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn get_instruments_fetches_the_supported_instruments() {
        let client = Client::default();

        let res = client.get_instruments().send().await;

        dbg!(&res);
    }
}
