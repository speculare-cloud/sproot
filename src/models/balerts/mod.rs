mod alerts;
mod alerts_impl;
pub use alerts::*;
pub use alerts_impl::*;

mod incidents;
mod incidents_impl;
pub use incidents::*;
pub use incidents_impl::*;

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
