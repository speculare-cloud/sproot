use crate::models::schema::hosts;

use serde::{Deserialize, Serialize};

/// DB Specific struct for hosts table
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = hosts)]
#[diesel(primary_key(uuid))]
pub struct Host {
    pub system: String,
    pub os_version: String,
    pub hostname: String,
    pub uptime: i64,
    pub uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[diesel(table_name = hosts)]
pub struct HostDTO<'a> {
    pub system: &'a str,
    pub os_version: &'a str,
    pub hostname: &'a str,
    pub uptime: i64,
    pub uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
