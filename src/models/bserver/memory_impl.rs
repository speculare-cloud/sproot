use diesel::{
    sql_types::{Text, Timestamp},
    *,
};

use super::{BaseMetrics, CFrom, Memory, MemoryDTO, MemoryDTORaw};
use crate::models::schema::memory::dsl::{created_at, host_uuid, memory as dsl_memory};
use crate::models::{get_granularity, HttpHost};
use crate::ConnType;
use crate::{apierrors::ApiError, models::get_aggregated_views};

impl BaseMetrics for Memory {
    type VecReturn = Vec<Memory>;

    type VecRawReturn = Vec<MemoryDTORaw>;

    fn get(
        conn: &mut ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Self::VecReturn, ApiError> {
        Ok(dsl_memory
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
					free,
					used,
					buffers,
					cached,
					time as created_at
				FROM memory{}
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
                    avg(free)::int8 as free,
                    avg(used)::int8 as used,
                    avg(buffers)::int8 as buffers,
                    avg(cached)::int8 as cached,
                    time_bucket('{}s', created_at) as time
                FROM memory
                WHERE host_uuid=$1 AND created_at BETWEEN $2 AND $3
                GROUP BY time ORDER BY time DESC
            )
            SELECT
                free,
                used,
                buffers,
                cached,
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

impl<'a> CFrom<&'a HttpHost> for MemoryDTO<'a> {
    type RET = Self;
    type UUID = &'a str;

    fn cfrom(item: &'a HttpHost, huuid: Self::UUID) -> Option<Self::RET> {
        let memory = item.memory.as_ref()?;

        Some(Self {
            total: memory.total as i64,
            free: memory.free as i64,
            used: memory.used as i64,
            shared: memory.shared as i64,
            buffers: memory.buffers as i64,
            cached: memory.cached as i64,
            host_uuid: huuid,
            created_at: item.created_at,
        })
    }
}
