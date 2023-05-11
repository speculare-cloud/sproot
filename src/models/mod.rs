use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{apierrors::ApiError, ConnType, Pool};

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

#[inline]
pub fn get_aggregated_views<'a>(size: i64) -> &'a str {
    // If less than 96h, then use the 10m aggregated (one data point per 10m)
    if size < 345600 {
        "_10m"
    } else {
        "_30m"
    }
}

pub trait BaseCrud<'a> {
    type RetType;
    type VecRetType;
    type TargetType;
    type UuidType;

    fn get(
        conn: &mut ConnType,
        uuid: Self::UuidType,
        size: i64,
        page: i64,
    ) -> Result<Self::VecRetType, ApiError>;

    fn get_specific(
        conn: &mut ConnType,
        target_id: Self::TargetType,
    ) -> Result<Self::RetType, ApiError>;
}

pub trait ExtCrud<'a> {
    type UuidType;

    fn count(conn: &mut ConnType, uuid: Self::UuidType, size: i64) -> Result<i64, ApiError>;
}

pub trait DtoBase<'a> {
    type GetReturn;
    type InsertType;
    type UpdateType;
    type TargetType;
    type UpdateReturnType;

    fn insert(conn: &mut ConnType, value: Self::InsertType) -> Result<usize, ApiError>;

    fn insert_and_get(
        conn: &mut ConnType,
        value: Self::InsertType,
    ) -> Result<Self::GetReturn, ApiError>;

    fn update(
        conn: &mut ConnType,
        target_id: Self::TargetType,
        value: Self::UpdateType,
    ) -> Result<usize, ApiError>;

    fn update_and_get(
        conn: &mut ConnType,
        target_id: Self::TargetType,
        value: Self::UpdateType,
    ) -> Result<Self::UpdateReturnType, ApiError>;

    fn delete(conn: &mut ConnType, target_id: Self::TargetType) -> Result<usize, ApiError>;
}
