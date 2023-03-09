use diesel::{
    sql_types::{Text, Timestamp},
    *,
};

use super::{BaseMetrics, CFrom, CpuStats, CpuStatsDTO, CpuStatsDTORaw};
use crate::models::schema::cpustats::dsl::{cpustats as dsl_cpustats, created_at, host_uuid};
use crate::models::{get_granularity, HttpHost};
use crate::ConnType;
use crate::{apierrors::ApiError, models::get_aggregated_views};

impl BaseMetrics for CpuStats {
    type VecReturn = Vec<CpuStats>;

    type VecRawReturn = Vec<CpuStatsDTORaw>;

    fn get(
        conn: &mut ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Self::VecReturn, ApiError> {
        Ok(dsl_cpustats
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(created_at.desc())
            .load(conn)?)
    }

    fn get_dated(
        conn: &mut ConnType,
        uuid: &str,
        min_date: chrono::NaiveDateTime,
        max_date: chrono::NaiveDateTime,
    ) -> Result<Self::VecRawReturn, ApiError> {
        let size = (max_date - min_date).num_seconds();
        let granularity = get_granularity(size);

        // If we're out of the get_granularity function capacity
        // use the "hardcoded" continuous_aggregated views from
        // TimescaleDB as defined in get_aggregated_views
        if granularity > 60 {
            let view = get_aggregated_views(size);

            // Execute an alternative SQL query
            return Ok(sql_query(format!(
                "
				SELECT
					interrupts,
					ctx_switches,
					soft_interrupts,
					processes,
					procs_running,
					procs_blocked,
					time as created_at
				FROM cpustats{}
				WHERE host_uuid=$1 AND time BETWEEN $2 AND $3
				ORDER BY time DESC",
                view
            ))
            .bind::<Text, _>(uuid)
            .bind::<Timestamp, _>(min_date)
            .bind::<Timestamp, _>(max_date)
            .load(conn)?);
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

impl<'a> CFrom<&'a HttpHost> for CpuStatsDTO<'a> {
    type RET = Self;
    type UUID = &'a str;

    fn cfrom(item: &'a HttpHost, huuid: Self::UUID) -> Option<Self::RET> {
        let cpustats = item.cpu_stats.as_ref()?;

        Some(Self {
            interrupts: cpustats.interrupts as i64,
            ctx_switches: cpustats.ctx_switches as i64,
            soft_interrupts: cpustats.soft_interrupts as i64,
            processes: cpustats.processes as i64,
            procs_running: cpustats.procs_running as i64,
            procs_blocked: cpustats.procs_blocked as i64,
            host_uuid: huuid,
            created_at: item.created_at,
        })
    }
}
