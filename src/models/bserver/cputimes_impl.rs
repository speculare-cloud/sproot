use diesel::{
    sql_types::{Text, Timestamp},
    *,
};

use super::{BaseMetrics, CFrom, CpuTimes, CpuTimesDTO, CpuTimesDTORaw};
use crate::apierrors::ApiError;
use crate::models::schema::cputimes::dsl::{cputimes as dsl_cputimes, created_at, host_uuid};
use crate::models::{get_granularity, HttpHost};
use crate::ConnType;

impl BaseMetrics for CpuTimes {
    type VecReturn = Vec<CpuTimes>;

    type VecRawReturn = Vec<CpuTimesDTORaw>;

    fn get(
        conn: &mut ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Self::VecReturn, ApiError> {
        Ok(dsl_cputimes
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
                    avg(cuser)::int8 as cuser,
                    avg(nice)::int8 as nice,
                    avg(system)::int8 as system,
                    avg(idle)::int8 as idle,
                    avg(iowait)::int8 as iowait,
                    avg(irq)::int8 as irq,
                    avg(softirq)::int8 as softirq,
                    avg(steal)::int8 as steal,
                    time_bucket('{}s', created_at) as time
                FROM cputimes
                WHERE host_uuid=$1 AND created_at BETWEEN $2 AND $3
                GROUP BY time ORDER BY time DESC
            )
            SELECT
                cuser,
                nice,
                system,
                idle,
                iowait,
                irq,
                softirq,
                steal,
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

impl<'a> CFrom<&'a HttpHost> for CpuTimesDTO<'a> {
    type RET = Self;
    type UUID = &'a str;

    fn cfrom(item: &'a HttpHost, huuid: Self::UUID) -> Option<Self::RET> {
        let cputimes = item.cpu_times.as_ref()?;

        Some(Self {
            cuser: cputimes.user as i64,
            nice: cputimes.nice as i64,
            system: cputimes.system as i64,
            idle: cputimes.idle as i64,
            iowait: cputimes.iowait as i64,
            irq: cputimes.irq as i64,
            softirq: cputimes.softirq as i64,
            steal: cputimes.steal as i64,
            guest: cputimes.guest as i64,
            guest_nice: cputimes.guest_nice as i64,
            host_uuid: huuid,
            created_at: item.created_at,
        })
    }
}
