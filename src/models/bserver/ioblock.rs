use crate::models::schema::ioblocks;

use diesel::{sql_types::Int8, *};
use serde::{Deserialize, Serialize};

/// DB Specific struct for ioblocks table
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name = ioblocks)]
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

#[derive(Queryable, QueryableByName, Serialize)]
pub struct IoBlockCount {
    #[diesel(sql_type = Int8)]
    pub count: i64,
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
