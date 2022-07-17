use diesel::{
    sql_types::{Text, Timestamp},
    *,
};

use super::{BaseMetrics, CFrom, Swap, SwapDTO, SwapDTORaw};
use crate::apierrors::ApiError;
use crate::models::schema::swap::dsl::{created_at, host_uuid, swap as dsl_swap};
use crate::models::{get_granularity, HttpHost};
use crate::ConnType;

impl BaseMetrics for Swap {
    type VecReturn = Vec<Swap>;

    type VecRawReturn = Vec<SwapDTORaw>;

    fn get(
        conn: &mut ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Self::VecReturn, ApiError> {
        Ok(dsl_swap
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

        // Prepare and run the query
        Ok(sql_query(format!(
            "
            WITH s AS (
                SELECT
                    avg(total)::int8 as total,
                    avg(free)::int8 as free,
                    avg(used)::int8 as used,
                    time_bucket('{}s', created_at) as time
                FROM swap
                WHERE host_uuid=$1 AND created_at BETWEEN $2 AND $3
                GROUP BY time ORDER BY time DESC
            )
            SELECT
                total,
                free,
                used,
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

impl<'a> CFrom<&'a HttpHost> for SwapDTO<'a> {
    type RET = Self;
    type UUID = &'a str;

    fn cfrom(item: &'a HttpHost, huuid: Self::UUID) -> Option<Self::RET> {
        let swap = item.swap.as_ref()?;
        Some(Self {
            total: swap.total as i64,
            free: swap.free as i64,
            used: swap.used as i64,
            host_uuid: huuid,
            created_at: item.created_at,
        })
    }
}
