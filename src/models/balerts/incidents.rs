use diesel::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::models::schema::incidents;

use super::Alerts;

/// Struct to hold information about incidents
/// Yes it's a lot of duplicate from the Alerts but as the Alerts can be updated
/// we need to store a snapshot of the configuration of the said alerts at the
/// time the incidents was created.
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Clone, TS)]
#[diesel(table_name = incidents)]
#[ts(export)]
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
    pub alerts_id: i64,
    pub cid: Uuid,
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
    pub alerts_id: i64,
    pub cid: Uuid,
}

/// Using a specific struct for the Update allow us to pass all as None expect the fields we want to update
#[derive(AsChangeset, Deserialize, Serialize, Debug, Default)]
#[diesel(table_name = incidents)]
pub struct IncidentsDTOUpdate {
    pub result: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub resolved_at: Option<chrono::NaiveDateTime>,
    pub status: Option<i32>,
    pub severity: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IncidentsJoined {
    #[serde(flatten)]
    pub incident: Incidents,
    pub alert: Alerts,
}

impl From<(Incidents, Alerts)> for IncidentsJoined {
    fn from(v: (Incidents, Alerts)) -> Self {
        Self {
            incident: v.0,
            alert: v.1,
        }
    }
}
