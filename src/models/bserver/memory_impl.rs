use super::{Memory, MemoryDTO, MemoryDTORaw};

use crate::apierrors::ApiError;
use crate::models::schema::memory::dsl::{created_at, host_uuid, memory as dsl_memory};
use crate::models::{get_granularity, HttpPostHost};
use crate::ConnType;

use diesel::{
    sql_types::{Text, Timestamp},
    *,
};

impl Memory {
    /// Return a Vector of Memory
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get Memory of
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn get_data(
        conn: &mut ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, ApiError> {
        Ok(dsl_memory
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(created_at.desc())
            .load(conn)?)
    }

    /// Return a Vector of Memory between min_date and max_date
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get Memory of
    /// * `size` - The number of elements to fetch
    /// * `min_date` - Min timestamp for the data to be fetched
    /// * `max_date` - Max timestamp for the data to be fetched
    pub fn get_data_dated(
        conn: &mut ConnType,
        uuid: &str,
        min_date: chrono::NaiveDateTime,
        max_date: chrono::NaiveDateTime,
    ) -> Result<Vec<MemoryDTORaw>, ApiError> {
        let size = (max_date - min_date).num_seconds();
        let granularity = get_granularity(size);

        // Dummy require to ensure no issue if table name change.
        // If the table's name is to be changed, we have to change it from the sql_query below.
        {
            #[allow(unused_imports)]
            use crate::models::schema::memory;
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

impl<'a> MemoryDTO<'a> {
    pub fn cfrom(item: &'a HttpPostHost, huuid: &'a str) -> Option<MemoryDTO<'a>> {
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
