use super::{CpuTimes, CpuTimesDTO, CpuTimesDTORaw};

use crate::errors::AppError;
use crate::models::schema::cputimes::dsl::{cputimes as dsl_cputimes, created_at, host_uuid};
use crate::models::{get_granularity, HttpPostHost};
use crate::ConnType;

use diesel::{
    sql_types::{Text, Timestamp},
    *,
};

impl CpuTimes {
    /// Return a Vector of CpuTimes
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get CpuTimes of
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn get_data(
        conn: &mut ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, AppError> {
        Ok(dsl_cputimes
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
    ) -> Result<Vec<CpuTimesDTORaw>, AppError> {
        let size = (max_date - min_date).num_seconds();
        let granularity = get_granularity(size);

        // Dummy require to ensure no issue if table name change.
        // If the table's name is to be changed, we have to change it from the sql_query below.
        {
            #[allow(unused_imports)]
            use crate::models::schema::cputimes;
        }

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

impl<'a> CpuTimesDTO<'a> {
    pub fn cfrom(item: &'a HttpPostHost, huuid: &'a str) -> Option<CpuTimesDTO<'a>> {
        let cputimes = item.cpu_times.as_ref()?;
        Some(Self {
            cuser: cputimes.user,
            nice: cputimes.nice,
            system: cputimes.system,
            idle: cputimes.idle,
            iowait: cputimes.iowait,
            irq: cputimes.irq,
            softirq: cputimes.softirq,
            steal: cputimes.steal,
            guest: cputimes.guest,
            guest_nice: cputimes.guest_nice,
            host_uuid: huuid,
            created_at: item.created_at,
        })
    }
}
