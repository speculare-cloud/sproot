use diesel::{
    sql_types::{Text, Timestamp},
    *,
};

use super::{BaseMetrics, CFrom, IoBlock, IoBlockDTO, IoBlockDTORaw};
use crate::apierrors::ApiError;
use crate::models::schema::ioblocks::dsl::{created_at, host_uuid, ioblocks as dsl_ioblocks};
use crate::models::{get_aggregated_views, get_granularity, HttpHost};
use crate::ConnType;

impl BaseMetrics for IoBlock {
    type VecReturn = Vec<IoBlock>;

    type VecRawReturn = Vec<IoBlockDTORaw>;

    fn get(
        conn: &mut ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Self::VecReturn, ApiError> {
        Ok(dsl_ioblocks
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
					device_name,
					read_bytes,
					write_bytes,
					time as created_at
				FROM ioblocks{}
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
}

impl<'a> CFrom<&'a HttpHost> for IoBlockDTO<'a> {
    type RET = Vec<Self>;
    type UUID = &'a str;

    fn cfrom(item: &'a HttpHost, huuid: Self::UUID) -> Option<Self::RET> {
        let ioblocks = item.ioblocks.as_ref()?;
        let mut list = Vec::with_capacity(ioblocks.len());

        for iostat in ioblocks {
            list.push(Self {
                device_name: &iostat.device_name,
                read_count: iostat.read_count as i64,
                read_bytes: iostat.read_bytes as i64,
                write_count: iostat.write_count as i64,
                write_bytes: iostat.write_bytes as i64,
                busy_time: iostat.busy_time as i64,
                host_uuid: huuid,
                created_at: item.created_at,
            })
        }
        Some(list)
    }
}
