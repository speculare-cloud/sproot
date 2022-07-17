use diesel::{sql_types::Int8, *};
use serde::{Deserialize, Serialize};

use crate::models::schema::ionets;

/// DB Specific struct for ionets table
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name = ionets)]
pub struct IoNet {
    pub id: i64,
    pub interface: String,
    pub rx_bytes: i64,
    pub rx_packets: i64,
    pub rx_errs: i64,
    pub rx_drop: i64,
    pub tx_bytes: i64,
    pub tx_packets: i64,
    pub tx_errs: i64,
    pub tx_drop: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, QueryableByName, Serialize)]
#[diesel(table_name = ionets)]
pub struct IoNetDTORaw {
    pub interface: String,
    pub rx_bytes: i64,
    pub tx_bytes: i64,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, QueryableByName, Serialize)]
pub struct IoNetCount {
    #[diesel(sql_type = Int8)]
    pub count: i64,
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[diesel(table_name = ionets)]
pub struct IoNetDTO<'a> {
    pub interface: &'a str,
    pub rx_bytes: i64,
    pub rx_packets: i64,
    pub rx_errs: i64,
    pub rx_drop: i64,
    pub tx_bytes: i64,
    pub tx_packets: i64,
    pub tx_errs: i64,
    pub tx_drop: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
