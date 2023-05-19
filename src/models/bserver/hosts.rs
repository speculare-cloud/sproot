use serde::{Deserialize, Serialize};
use sys_metrics::{
    cpu::{CpuStats, CpuTimes, LoadAvg},
    disks::{Disks, IoBlock},
    memory::{Memory, Swap},
    network::IoNet,
};
use ts_rs::TS;

use crate::models::schema::hosts;

/// DB Specific struct for hosts table
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, AsChangeset, TS)]
#[diesel(table_name = hosts)]
#[diesel(primary_key(uuid))]
#[ts(export)]
pub struct Host {
    pub system: String,
    pub os_version: String,
    pub hostname: String,
	#[ts(type = "number")]
    pub uptime: i64,
    pub uuid: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
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

// ===============
// HTTP POST model
// ===============
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpHost {
    pub system: String,
    pub os_version: String,
    pub hostname: String,
    pub uptime: i64,
    pub cpu_stats: Option<CpuStats>,
    pub cpu_times: Option<CpuTimes>,
    pub load_avg: Option<LoadAvg>,
    pub disks: Option<Vec<Disks>>,
    pub ioblocks: Option<Vec<IoBlock>>,
    pub memory: Option<Memory>,
    pub swap: Option<Swap>,
    pub ionets: Option<Vec<IoNet>>,
    pub created_at: chrono::NaiveDateTime,
}
