use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::schema::ioblocks;

/// DB Specific struct for ioblocks table
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, TS)]
#[diesel(table_name = ioblocks)]
#[ts(export)]
pub struct IoBlock {
    pub id: i64,
    pub device_name: String,
    pub read_count: i64,
    pub read_bytes: i64,
    pub write_count: i64,
    pub write_bytes: i64,
    pub busy_time: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, QueryableByName, Serialize)]
#[diesel(table_name = ioblocks)]
pub struct IoBlockDTORaw {
    pub device_name: String,
    pub read_bytes: i64,
    pub write_bytes: i64,
    pub created_at: chrono::NaiveDateTime,
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[diesel(table_name = ioblocks)]
pub struct IoBlockDTO<'a> {
    pub device_name: &'a str,
    pub read_count: i64,
    pub read_bytes: i64,
    pub write_count: i64,
    pub write_bytes: i64,
    pub busy_time: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
