use diesel::{
    dsl::count_distinct,
    sql_types::{Text, Timestamp},
    *,
};

use super::{BaseMetrics, CFrom, ExtMetrics, IoNet, IoNetDTO, IoNetDTORaw};
use crate::models::{
    get_aggregated_views,
    schema::ionets::dsl::{created_at, host_uuid, ionets as dsl_ionets},
};
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

        // If we're out of the get_granularity function capacity
        // use the "hardcoded" continuous_aggregated views from
        // TimescaleDB as defined in get_aggregated_views
        if granularity > 60 {
            let view = get_aggregated_views(size);

            // Execute an alternative SQL query
            return Ok(sql_query(format!(
                "
				SELECT
					interface,
					rx_bytes,
					tx_bytes,
					time as created_at
				FROM ionets{}
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
    fn count_unique(
        conn: &mut ConnType,
        uuid: &str,
        min_date: chrono::NaiveDateTime,
        max_date: chrono::NaiveDateTime,
    ) -> Result<i64, ApiError> {
        Ok(dsl_ionets
            .select(count_distinct(interface))
            .filter(
                host_uuid
                    .eq(uuid)
                    .and(created_at.between(min_date, max_date)),
            )
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
