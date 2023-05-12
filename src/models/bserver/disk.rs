use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::schema::disks;

/// DB Specific struct for disks table
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, TS)]
#[diesel(table_name = disks)]
#[ts(export)]
pub struct Disk {
    pub id: i64,
    pub disk_name: String,
    pub mount_point: String,
    pub total_space: i64,
    pub avail_space: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, QueryableByName, Serialize)]
#[diesel(table_name = disks)]
pub struct DiskDTORaw {
    pub disk_name: String,
    pub total_space: i64,
    pub avail_space: i64,
    pub created_at: chrono::NaiveDateTime,
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[diesel(table_name = disks)]
pub struct DiskDTO<'a> {
    pub disk_name: &'a str,
    pub mount_point: &'a str,
    pub total_space: i64,
    pub avail_space: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
