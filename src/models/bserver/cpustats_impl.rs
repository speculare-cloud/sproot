use super::{CpuStats, CpuStatsDTO, CpuStatsDTORaw};

use crate::errors::AppError;
use crate::models::schema::cpustats::dsl::{cpustats as dsl_cpustats, created_at, host_uuid};
use crate::models::{get_granularity, HttpPostHost};
use crate::ConnType;

use diesel::{
    sql_types::{Text, Timestamp},
    *,
};

impl CpuStats {
    /// Return a Vector of CpuStats
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get CpuStats of
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn get_data(
        conn: &mut ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, AppError> {
        Ok(dsl_cpustats
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(created_at.desc())
            .load(conn)?)
    }

    /// Return a Vector of CpuTimes between min_date and max_date
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get CpuTimes of
    /// * `size` - The number of elements to fetch
    /// * `min_date` - Min timestamp for the data to be fetched
    /// * `max_date` - Max timestamp for the data to be fetched
    pub fn get_data_dated(
        conn: &mut ConnType,
        uuid: &str,
        min_date: chrono::NaiveDateTime,
        max_date: chrono::NaiveDateTime,
    ) -> Result<Vec<CpuStatsDTORaw>, AppError> {
        let size = (max_date - min_date).num_seconds();
        let granularity = get_granularity(size);

        // Dummy require to ensure no issue if table name change.
        // If the table's name is to be changed, we have to change it from the sql_query below.
        {
            #[allow(unused_imports)]
            use crate::models::schema::cpustats;
        }

        // Prepare and run the query
        Ok(sql_query(format!(
            "
            WITH s AS (
                SELECT
                    avg(interrupts)::int8 as interrupts,
                    avg(ctx_switches)::int8 as ctx_switches,
                    avg(soft_interrupts)::int8 as soft_interrupts,
                    avg(processes)::int8 as processes,
                    avg(procs_running)::int8 as procs_running,
                    avg(procs_blocked)::int8 as procs_blocked,
                    time_bucket('{}s', created_at) as time
                FROM cpustats
                WHERE host_uuid=$1 AND created_at BETWEEN $2 AND $3
                GROUP BY time ORDER BY time DESC
            )
            SELECT
                interrupts,
                ctx_switches,
                soft_interrupts,
                processes,
                procs_running,
                procs_blocked,
                time as created_at
            FROM s",
            granularity
        ))
        .bind::<Text, _>(uuid)
        .bind::<Timestamp, _>(min_date)
        .bind::<Timestamp, _>(max_date)
        .load(conn)?)
    }
}

impl<'a> CpuStatsDTO<'a> {
    pub fn cfrom(item: &'a HttpPostHost, huuid: &'a str) -> Option<CpuStatsDTO<'a>> {
        let cpustats = item.cpu_stats.as_ref()?;
        Some(Self {
            interrupts: cpustats.interrupts,
            ctx_switches: cpustats.ctx_switches,
            soft_interrupts: cpustats.soft_interrupts,
            processes: cpustats.processes,
            procs_running: cpustats.procs_running,
            procs_blocked: cpustats.procs_blocked,
            host_uuid: huuid,
            created_at: item.created_at,
        })
    }
}
