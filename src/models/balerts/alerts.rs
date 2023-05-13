use diesel::{sql_types::BigInt, *};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::models::schema::alerts;

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Clone, TS)]
#[diesel(table_name = alerts)]
#[ts(export)]
pub struct Alerts {
    pub id: i64,
    pub active: bool,
    #[diesel(column_name = _name)]
    pub name: String,
    #[diesel(column_name = _table)]
    pub table: String,
    // Represent the query used to check the alarms against the database's data
    // eg: "avg pct 10m of w,x over y,z"
    //     =>(will compute the (10m avg(w)+avg(x) over avg(y)+avg(z)) * 100, result is in percentage as asked using percentage and over)
    // eg: "avg abs 10m of x"
    //     =>(will compute based on only an absolute value (no division))
    pub lookup: String,
    // Number of seconds between checks
    pub timing: i32,
    // $this > 50 ($this refer to the result of the query, should return a bool)
    pub warn: String,
    // $this > 80 ($this refer to the result of the query, should return a bool)
    pub crit: String,
    // Description of the alarms
    pub info: Option<String>,
    // Targeted host
    pub host_uuid: String,
    // The "owner" of the Alert
    pub cid: Uuid,
    // Targeted hostname
    pub hostname: String,
    // Where SQL condition
    pub where_clause: Option<String>,
}

#[derive(AsChangeset, Deserialize, Serialize, Debug, Default, TS)]
#[diesel(table_name = alerts)]
#[ts(export)]
pub struct AlertsDTOUpdate {
    pub active: Option<bool>,
    #[diesel(column_name = _name)]
    pub name: Option<String>,
    #[diesel(column_name = _table)]
    pub table: Option<String>,
    pub lookup: Option<String>,
    pub timing: Option<i32>,
    pub warn: Option<String>,
    pub crit: Option<String>,
    pub info: Option<String>,
    pub where_clause: Option<String>,
}

#[derive(Queryable, QueryableByName, Deserialize, Serialize, Debug, Default, TS)]
#[diesel(table_name = alerts)]
#[ts(export)]
pub struct HttpAlertsCount {
    #[diesel(sql_type = BigInt)]
    pub active: i64,
    #[diesel(sql_type = BigInt)]
    pub inactive: i64,
    #[diesel(sql_type = BigInt)]
    pub total: i64,
}

// ================
// Insertable model
// ================
#[derive(Insertable, Deserialize, Serialize, Debug, Default, TS)]
#[diesel(table_name = alerts)]
#[ts(export)]
pub struct AlertsDTO {
    pub active: Option<bool>,
    #[diesel(column_name = _name)]
    pub name: String,
    #[diesel(column_name = _table)]
    pub table: String,
    pub lookup: String,
    pub timing: i32,
    pub warn: String,
    pub crit: String,
    pub info: String,
    pub host_uuid: String,
    pub cid: Uuid,
    pub hostname: String,
    pub where_clause: String,
}
