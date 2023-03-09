use diesel::dsl::count_distinct;
use diesel::{
    sql_types::{Text, Timestamp},
    *,
};

use super::{BaseMetrics, CFrom, Disk, DiskDTO, DiskDTORaw, ExtMetrics};
use crate::apierrors::ApiError;
use crate::models::schema::disks::dsl::{created_at, disk_name, disks as dsl_disks, host_uuid};
use crate::models::{get_aggregated_views, get_granularity, HttpHost};
use crate::ConnType;

impl BaseMetrics for Disk {
    type VecReturn = Vec<Disk>;

    type VecRawReturn = Vec<DiskDTORaw>;

    fn get(
        conn: &mut ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Self::VecReturn, ApiError> {
        Ok(dsl_disks
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
					disk_name,
					total_space,
					avail_space,
					time as created_at
				FROM disks{}
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
                    disk_name,
                    avg(total_space)::int8 as total_space,
                    avg(avail_space)::int8 as avail_space,
                    time_bucket('{}s', created_at) as time
                FROM disks
                WHERE host_uuid=$1 AND created_at BETWEEN $2 AND $3
                GROUP BY time,disk_name ORDER BY time DESC
            )
            SELECT
                disk_name,
                total_space,
                avail_space,
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

impl ExtMetrics for Disk {
    fn count_unique(
        conn: &mut ConnType,
        uuid: &str,
        min_date: chrono::NaiveDateTime,
        max_date: chrono::NaiveDateTime,
    ) -> Result<i64, ApiError> {
        Ok(dsl_disks
            .select(count_distinct(disk_name))
            .filter(
                host_uuid
                    .eq(uuid)
                    .and(created_at.between(min_date, max_date)),
            )
            .first(conn)?)
    }
}

impl<'a> CFrom<&'a HttpHost> for DiskDTO<'a> {
    type RET = Vec<DiskDTO<'a>>;
    type UUID = &'a str;

    fn cfrom(item: &'a HttpHost, huuid: Self::UUID) -> Option<Self::RET> {
        let disks = item.disks.as_ref()?;
        let mut list = Vec::with_capacity(disks.len());

        for disk in disks {
            list.push(Self {
                disk_name: &disk.name,
                mount_point: &disk.mount_point,
                total_space: disk.total_space as i64,
                avail_space: disk.avail_space as i64,
                host_uuid: huuid,
                created_at: item.created_at,
            })
        }
        Some(list)
    }
}
