use diesel::{
    sql_types::{Int8, Text, Timestamp},
    *,
};

use super::{CFrom, IoNet, IoNetCount, IoNetDTO, IoNetDTORaw};
use crate::apierrors::ApiError;
use crate::models::schema::ionets::dsl::{created_at, host_uuid, ionets as dsl_ionets};
use crate::models::{get_granularity, HttpHost};
use crate::ConnType;

impl IoNet {
    /// Return a Vector of IoNet
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get IoNet of
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn get_data(
        conn: &mut ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, ApiError> {
        Ok(dsl_ionets
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(created_at.desc())
            .load(conn)?)
    }

    /// Return a Vector of IoNet between min_date and max_date
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get IoNet of
    /// * `size` - The number of elements to fetch
    /// * `min_date` - Min timestamp for the data to be fetched
    /// * `max_date` - Max timestamp for the data to be fetched
    pub fn get_data_dated(
        conn: &mut ConnType,
        uuid: &str,
        min_date: chrono::NaiveDateTime,
        max_date: chrono::NaiveDateTime,
    ) -> Result<Vec<IoNetDTORaw>, ApiError> {
        let size = (max_date - min_date).num_seconds();
        let granularity = get_granularity(size);

        // Dummy require to ensure no issue if table name change.
        // If the table's name is to be changed, we have to change it from the sql_query below.
        {
            #[allow(unused_imports)]
            use crate::models::schema::ionets;
        }

        // Prepare and run the query
        Ok(sql_query(format!(
            "
            WITH s AS (
                SELECT
                    interface,
                    avg(rx_bytes)::int8 as rx_bytes,
                    avg(tx_bytes)::int8 as tx_bytes,
                    time_bucket('{}s', created_at) as time
                FROM ionets
                WHERE host_uuid=$1 AND created_at BETWEEN $2 AND $3
                GROUP BY time,interface ORDER BY time DESC
            )
            SELECT
                interface,
                rx_bytes,
                tx_bytes,
                time as created_at
            FROM s",
            granularity
        ))
        .bind::<Text, _>(uuid)
        .bind::<Timestamp, _>(min_date)
        .bind::<Timestamp, _>(max_date)
        .load(conn)?)
    }

    /// Return the numbers of IoNet the host have
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get the number of IoNet of
    /// * `size` - The number of elements to fetch
    pub fn count(conn: &mut ConnType, uuid: &str, size: i64) -> Result<i64, ApiError> {
        // Dummy require to ensure no issue if table name change.
        // If the table's name is to be changed, we have to change it from the sql_query below.
        {
            #[allow(unused_imports)]
            use crate::models::schema::ionets;
        }

        let res = sql_query(
            "
            WITH s AS
                (SELECT id, interface, created_at
                    FROM ionets
                    WHERE host_uuid=$1
                    ORDER BY created_at
                    DESC LIMIT $2
                )
            SELECT
                COUNT(DISTINCT interface)
                FROM s",
        )
        .bind::<Text, _>(uuid)
        .bind::<Int8, _>(size)
        .load::<IoNetCount>(conn)?;

        if res.is_empty() {
            Ok(0)
        } else {
            Ok(res[0].count)
        }
    }
}

impl<'a> CFrom<&'a HttpHost> for IoNetDTO<'a> {
    type RET = Vec<Self>;
    type UUID = &'a str;

    fn cfrom(item: &'a HttpHost, huuid: Self::UUID) -> Option<Self::RET> {
        let ionets = item.ionets.as_ref()?;
        let mut list = Vec::with_capacity(ionets.len());

        for iocounter in ionets {
            list.push(Self {
                interface: &iocounter.interface,
                rx_bytes: iocounter.rx_bytes as i64,
                rx_packets: iocounter.rx_packets as i64,
                rx_errs: iocounter.rx_errs as i64,
                rx_drop: iocounter.rx_drop as i64,
                tx_bytes: iocounter.tx_bytes as i64,
                tx_packets: iocounter.tx_packets as i64,
                tx_errs: iocounter.tx_errs as i64,
                tx_drop: iocounter.tx_drop as i64,
                host_uuid: huuid,
                created_at: item.created_at,
            })
        }
        Some(list)
    }
}
