use diesel::{
    sql_types::{Text, Timestamp},
    *,
};

use super::{BaseMetrics, CFrom, LoadAvg, LoadAvgDTO, LoadAvgDTORaw};
use crate::apierrors::ApiError;
use crate::models::schema::loadavg::dsl::{created_at, host_uuid, loadavg as dsl_loadavg};
use crate::models::{get_granularity, HttpHost};
use crate::ConnType;

impl BaseMetrics for LoadAvg {
    type VecReturn = Vec<LoadAvg>;

    type VecRawReturn = Vec<LoadAvgDTORaw>;

    fn get(
        conn: &mut ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Self::VecReturn, ApiError> {
        Ok(dsl_loadavg
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
                    avg(one)::float8 as one,
                    avg(five)::float8 as five,
                    avg(fifteen)::float8 as fifteen,
                    time_bucket('{}s', created_at) as time
                FROM loadavg
                WHERE host_uuid=$1 AND created_at BETWEEN $2 AND $3
                GROUP BY time ORDER BY time DESC
            )
            SELECT
                one,
                five,
                fifteen,
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

impl<'a> CFrom<&'a HttpHost> for LoadAvgDTO<'a> {
    type RET = Self;
    type UUID = &'a str;

    fn cfrom(item: &'a HttpHost, huuid: Self::UUID) -> Option<Self::RET> {
        let load_avg = item.load_avg.as_ref()?;

        Some(Self {
            one: load_avg.one,
            five: load_avg.five,
            fifteen: load_avg.fifteen,
            host_uuid: huuid,
            created_at: item.created_at,
        })
    }
}
