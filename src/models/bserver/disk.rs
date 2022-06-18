use crate::models::schema::disks;

use diesel::{sql_types::Int8, *};
use serde::{Deserialize, Serialize};

/// DB Specific struct for disks table
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name = disks)]
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

#[derive(Queryable, QueryableByName, Serialize)]
pub struct DisksCount {
    #[diesel(sql_type = Int8)]
    pub count: i64,
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
