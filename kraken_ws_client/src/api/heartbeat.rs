use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HeartbeatEvent {
    pub channel: String,
}
