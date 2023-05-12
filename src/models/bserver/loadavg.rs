use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::schema::loadavg;

/// DB Specific struct for loadavg table
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, TS)]
#[diesel(table_name = loadavg)]
#[ts(export)]
pub struct LoadAvg {
    pub id: i64,
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, QueryableByName, Serialize)]
#[diesel(table_name = loadavg)]
pub struct LoadAvgDTORaw {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
    pub created_at: chrono::NaiveDateTime,
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[diesel(table_name = loadavg)]
pub struct LoadAvgDTO<'a> {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
