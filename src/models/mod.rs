use crate::Pool;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod balerts;
mod bauth;
mod bserver;
pub mod schema;

pub use balerts::*;
pub use bauth::*;
pub use bserver::*;

/// Used to hold the information for the database in Actix
pub struct MetricsPool {
    pub pool: Pool,
}

pub struct AuthPool {
    pub pool: Pool,
}

/// Represent the host_uuid for Actix extractor
#[derive(Debug, Serialize, Deserialize)]
pub struct Specific {
    pub uuid: String,
}

/// Dummy used to represent the user in Actix's extensions
#[derive(Clone, Debug)]
pub struct InnerUser {
    pub uuid: Uuid,
}

/// granularity == the range in which we'll group the data
/// We'll compute the granularity from this equation:
/// f(x) = ((0.00192859 * x) * (1.00694) + 0.298206);
/// which give us ~=:
///  size = 300 => 1
///  size = 900 => 2
///  size = 1800 => 5
///  size = 7200 => 20
///  size = 21600 => 60
/// which means for size = 21600 that we'll get the avg of each 60s intervals
#[inline]
pub fn get_granularity(size: i64) -> u32 {
    std::cmp::min(
        86400,
        std::cmp::max(1, ((0.003 * size as f32) * (0.93) + 0.298206) as u32),
    )
}
