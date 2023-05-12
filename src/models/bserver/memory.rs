use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::schema::memory;

/// DB Specific struct for memory table
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, TS)]
#[diesel(table_name = memory)]
#[ts(export)]
pub struct Memory {
    pub id: i64,
    pub total: i64,
    pub free: i64,
    pub used: i64,
    pub shared: i64,
    pub buffers: i64,
    pub cached: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, QueryableByName, Serialize)]
#[diesel(table_name = memory)]
pub struct MemoryDTORaw {
    pub free: i64,
    pub used: i64,
    pub buffers: i64,
    pub cached: i64,
    pub created_at: chrono::NaiveDateTime,
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[diesel(table_name = memory)]
pub struct MemoryDTO<'a> {
    pub total: i64,
    pub free: i64,
    pub used: i64,
    pub shared: i64,
    pub buffers: i64,
    pub cached: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
