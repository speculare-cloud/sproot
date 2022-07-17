use diesel::*;
use serde::{Deserialize, Serialize};

use crate::models::schema::incidents;

/// Struct to hold information about incidents
/// Yes it's a lot of duplicate from the Alerts but as the Alerts can be updated
/// we need to store a snapshot of the configuration of the said alerts at the
/// time the incidents was created.
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Clone)]
#[diesel(table_name = incidents)]
pub struct Incidents {
    pub id: i32,
    pub result: String,
    pub started_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub resolved_at: Option<chrono::NaiveDateTime>,
    pub host_uuid: String,
    pub hostname: String,
    pub status: i32,
    pub severity: i32,
    pub alerts_id: String,
}

/// Insertable struct (no id fields => which is auto generated)
#[derive(Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = incidents)]
pub struct IncidentsDTO {
    pub result: String,
    pub started_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub resolved_at: Option<chrono::NaiveDateTime>,
    pub host_uuid: String,
    pub hostname: String,
    pub status: i32,
    pub severity: i32,
    pub alerts_id: String,
}

/// Using a specific struct for the Update allow us to pass all as None expect the fields we want to update
#[derive(AsChangeset, Deserialize, Serialize, Debug, Default)]
#[diesel(table_name = incidents)]
pub struct IncidentsDTOUpdate {
    pub result: Option<String>,
    pub started_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub resolved_at: Option<chrono::NaiveDateTime>,
    pub host_uuid: Option<String>,
    pub hostname: Option<String>,
    pub status: Option<i32>,
    pub severity: Option<i32>,
    pub alerts_id: Option<String>,
}
