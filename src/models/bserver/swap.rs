use serde::{Deserialize, Serialize};

use crate::models::schema::swap;

/// DB Specific struct for swap table
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name = swap)]
pub struct Swap {
    pub id: i64,
    pub total: i64,
    pub free: i64,
    pub used: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, QueryableByName, Serialize)]
#[diesel(table_name = swap)]
pub struct SwapDTORaw {
    pub total: i64,
    pub free: i64,
    pub used: i64,
    pub created_at: chrono::NaiveDateTime,
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[diesel(table_name = swap)]
pub struct SwapDTO<'a> {
    pub total: i64,
    pub free: i64,
    pub used: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
