use diesel::{
    dsl::count_distinct,
    sql_types::{Text, Timestamp},
    *,
};

use super::{BaseMetrics, CFrom, ExtMetrics, IoNet, IoNetDTO, IoNetDTORaw};
use crate::models::schema::ionets::dsl::{created_at, host_uuid, ionets as dsl_ionets};
use crate::models::{get_granularity, HttpHost};
use crate::ConnType;
use crate::{apierrors::ApiError, models::schema::ionets::interface};

impl BaseMetrics for IoNet {
    type VecReturn = Vec<IoNet>;

    type VecRawReturn = Vec<IoNetDTORaw>;

    fn get(
        conn: &mut ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Self::VecReturn, ApiError> {
        Ok(dsl_ionets
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
}

impl ExtMetrics for IoNet {
    fn count_unique(conn: &mut ConnType, uuid: &str, size: i64) -> Result<i64, ApiError> {
        Ok(dsl_ionets
            .select(count_distinct(interface))
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .first(conn)?)
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
