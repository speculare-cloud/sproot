use super::{IoBlock, IoBlockCount, IoBlockDTO, IoBlockDTORaw};

use crate::errors::AppError;
use crate::models::schema::ioblocks::dsl::{created_at, host_uuid, ioblocks as dsl_ioblocks};
use crate::models::{get_granularity, HttpPostHost};
use crate::ConnType;

use diesel::{
    sql_types::{Int8, Text, Timestamp},
    *,
};

impl IoBlock {
    /// Return a Vector of IoBlock
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get IoBlock of
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn get_data(
        conn: &mut ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, AppError> {
        Ok(dsl_ioblocks
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(created_at.desc())
            .load(conn)?)
    }

    /// Return a Vector of IoBlock between min_date and max_date
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get IoBlock of
    /// * `size` - The number of elements to fetch
    /// * `min_date` - Min timestamp for the data to be fetched
    /// * `max_date` - Max timestamp for the data to be fetched
    pub fn get_data_dated(
        conn: &mut ConnType,
        uuid: &str,
        min_date: chrono::NaiveDateTime,
        max_date: chrono::NaiveDateTime,
    ) -> Result<Vec<IoBlockDTORaw>, AppError> {
        let size = (max_date - min_date).num_seconds();
        let granularity = get_granularity(size);

        // Dummy require to ensure no issue if table name change.
        // If the table's name is to be changed, we have to change it from the sql_query below.
        {
            #[allow(unused_imports)]
            use crate::models::schema::ioblocks;
        }

        // Prepare and run the query
        Ok(sql_query(format!(
            "
            WITH s AS (
                SELECT
                    device_name,
                    avg(read_bytes)::int8 as read_bytes,
                    avg(write_bytes)::int8 as write_bytes,
                    time_bucket('{}s', created_at) as time
                FROM ioblocks
                WHERE host_uuid=$1 AND created_at BETWEEN $2 AND $3
                GROUP BY time,device_name ORDER BY time DESC
            )
            SELECT
                device_name,
                read_bytes,
                write_bytes,
                time as created_at
            FROM s",
            granularity
        ))
        .bind::<Text, _>(uuid)
        .bind::<Timestamp, _>(min_date)
        .bind::<Timestamp, _>(max_date)
        .load(conn)?)
    }

    /// Return the numbers of ioblocks the host have
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get the number of ioblocks of
    /// * `size` - The number of elements to fetch
    pub fn count(conn: &mut ConnType, uuid: &str, size: i64) -> Result<i64, AppError> {
        // Dummy require to ensure no issue if table name change.
        // If the table's name is to be changed, we have to change it from the sql_query below.
        {
            #[allow(unused_imports)]
            use crate::models::schema::ioblocks;
        }

        let res = sql_query(
            "
            WITH s AS
                (SELECT id, device_name, created_at
                    FROM ioblocks
                    WHERE host_uuid=$1
                    ORDER BY created_at
                    DESC LIMIT $2
                )
            SELECT
                COUNT(DISTINCT device_name)
                FROM s",
        )
        .bind::<Text, _>(uuid)
        .bind::<Int8, _>(size)
        .load::<IoBlockCount>(conn)?;

        if res.is_empty() {
            Ok(0)
        } else {
            Ok(res[0].count)
        }
    }
}

impl<'a> IoBlockDTO<'a> {
    pub fn cfrom(item: &'a HttpPostHost, huuid: &'a str) -> Option<Vec<IoBlockDTO<'a>>> {
        let ioblocks = item.ioblocks.as_ref()?;
        let mut list = Vec::with_capacity(ioblocks.len());
        for iostat in ioblocks {
            list.push(Self {
                device_name: &iostat.device_name,
                read_count: iostat.read_count,
                read_bytes: iostat.read_bytes,
                write_count: iostat.write_count,
                write_bytes: iostat.write_bytes,
                busy_time: iostat.busy_time,
                host_uuid: huuid,
                created_at: item.created_at,
            })
        }
        Some(list)
    }
}