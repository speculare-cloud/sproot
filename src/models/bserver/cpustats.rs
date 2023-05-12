use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::schema::cpustats;

/// DB Specific struct for cpustats table
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, TS)]
#[diesel(table_name = cpustats)]
#[ts(export)]
pub struct CpuStats {
    pub id: i64,
    pub interrupts: i64,
    pub ctx_switches: i64,
    pub soft_interrupts: i64,
    pub processes: i64,
    pub procs_running: i64,
    pub procs_blocked: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, QueryableByName, Serialize)]
#[diesel(table_name = cpustats)]
pub struct CpuStatsDTORaw {
    pub interrupts: i64,
    pub ctx_switches: i64,
    pub soft_interrupts: i64,
    pub processes: i64,
    pub procs_running: i64,
    pub procs_blocked: i64,
    pub created_at: chrono::NaiveDateTime,
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[diesel(table_name = cpustats)]
pub struct CpuStatsDTO<'a> {
    pub interrupts: i64,
    pub ctx_switches: i64,
    pub soft_interrupts: i64,
    pub processes: i64,
    pub procs_running: i64,
    pub procs_blocked: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
