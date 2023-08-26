use crate::client::Event;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemStatus {
    CancelOnly,
    Maintenance,
    Online,
    PostOnly,
}

#[derive(Debug, Deserialize)]
pub struct StatusData {
    pub api_version: String,
    pub connection_id: i64,
    pub system: SystemStatus,
    pub version: String,
}

pub type StatusEvent = Event<Vec<StatusData>>;
