mod alerts;
pub use alerts::*;

mod incidents;
pub use incidents::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum AlertSource {
    #[serde(rename = "files")]
    Files,
    #[serde(rename = "database")]
    Database,
}

pub fn default_alertssource() -> AlertSource {
    AlertSource::Files
}
