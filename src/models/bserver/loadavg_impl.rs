use super::{LoadAvg, LoadAvgDTO, LoadAvgDTORaw};

use crate::apierrors::ApiError;
use crate::models::schema::loadavg::dsl::{created_at, host_uuid, loadavg as dsl_loadavg};
use crate::models::{get_granularity, HttpPostHost};
use crate::ConnType;

use diesel::{
    sql_types::{Text, Timestamp},
    *,
};

impl LoadAvg {
    /// Return a Vector of LoadAvg
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get LoadAvg of
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn get_data(
        conn: &mut ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, ApiError> {
        Ok(dsl_loadavg
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(created_at.desc())
            .load(conn)?)
    }

    /// Return a Vector of LoadAvg between min_date and max_date
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get LoadAvg of
    /// * `size` - The number of elements to fetch
    /// * `min_date` - Min timestamp for the data to be fetched
    /// * `max_date` - Max timestamp for the data to be fetched
    pub fn get_data_dated(
        conn: &mut ConnType,
        uuid: &str,
        min_date: chrono::NaiveDateTime,
        max_date: chrono::NaiveDateTime,
    ) -> Result<Vec<LoadAvgDTORaw>, ApiError> {
        let size = (max_date - min_date).num_seconds();
        let granularity = get_granularity(size);

        // Dummy require to ensure no issue if table name change.
        // If the table's name is to be changed, we have to change it from the sql_query below.
        {
            #[allow(unused_imports)]
            use crate::models::schema::loadavg;
        }

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

impl<'a> LoadAvgDTO<'a> {
    pub fn cfrom(item: &'a HttpPostHost, huuid: &'a str) -> Option<LoadAvgDTO<'a>> {
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
