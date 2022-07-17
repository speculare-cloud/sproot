use diesel::*;

use super::{
    CFrom, CpuStatsDTO, CpuTimesDTO, DiskDTO, Host, HostDTO, HttpHost, IoBlockDTO, IoNetDTO,
    LoadAvgDTO, MemoryDTO, SwapDTO,
};
use crate::apierrors::ApiError;
use crate::models::schema::{
    cpustats::dsl::*,
    cputimes::dsl::*,
    disks::dsl::*,
    hosts::dsl::{hosts as dsl_host, uuid, *},
    ioblocks::dsl::*,
    ionets::dsl::*,
    loadavg::dsl::*,
    memory::dsl::*,
    swap::dsl::*,
};
use crate::models::BaseCrud;
use crate::ConnType;

impl Host {
    pub fn insert(conn: &mut ConnType, items: &[HttpHost], huuid: &str) -> Result<(), ApiError> {
        let len = items.len();

        if len == 1 {
            return Self::insert_one(conn, &items[0], huuid);
        }

        if len == 0 {
            return Ok(());
        }

        // Even if this method (using Vec) use more memory, it prefer speed over low RAM usage.
        let mut v_ncpustats: Vec<CpuStatsDTO> = Vec::with_capacity(len);
        let mut v_ncputimes: Vec<CpuTimesDTO> = Vec::with_capacity(len);
        let mut v_nloadavg: Vec<LoadAvgDTO> = Vec::with_capacity(len);
        let mut v_nmemory: Vec<MemoryDTO> = Vec::with_capacity(len);
        let mut v_nswap: Vec<SwapDTO> = Vec::with_capacity(len);
        // For these vector we can't predict the lenght of them, as a server/computer can have
        // a new net interfaces or disks at any time. So we create regular Vec that will grow if needed.
        let mut v_ndisks: Vec<DiskDTO> = Vec::new();
        let mut v_nioblocks: Vec<IoBlockDTO> = Vec::new();
        let mut v_nionets: Vec<IoNetDTO> = Vec::new();

        for item in items {
            // Only insert Option if they are present to their Vector
            if let Some(value_cpustats) = CpuStatsDTO::cfrom(item, huuid) {
                v_ncpustats.push(value_cpustats);
            }
            if let Some(value_cputimes) = CpuTimesDTO::cfrom(item, huuid) {
                v_ncputimes.push(value_cputimes);
            }
            if let Some(value_loadavg) = LoadAvgDTO::cfrom(item, huuid) {
                v_nloadavg.push(value_loadavg);
            }
            if let Some(value_memory) = MemoryDTO::cfrom(item, huuid) {
                v_nmemory.push(value_memory);
            }
            if let Some(value_swap) = SwapDTO::cfrom(item, huuid) {
                v_nswap.push(value_swap);
            }
            if let Some(value_disks) = DiskDTO::cfrom(item, huuid).as_mut() {
                v_ndisks.append(value_disks);
            }
            if let Some(value_iostats) = IoBlockDTO::cfrom(item, huuid).as_mut() {
                v_nioblocks.append(value_iostats);
            }
            if let Some(value_iocounters) = IoNetDTO::cfrom(item, huuid).as_mut() {
                v_nionets.append(value_iocounters);
            }

            // Insert Host data, if conflict, only update uptime
            insert_into(hosts)
                .values(HostDTO::cfrom(item, huuid))
                .on_conflict(uuid)
                .do_update()
                .set(uptime.eq(item.uptime))
                .execute(conn)?;
        }

        // Insert Vec of Table from the for loop in one call (66% faster)
        insert_into(cpustats).values(&v_ncpustats).execute(conn)?;
        insert_into(cputimes).values(&v_ncputimes).execute(conn)?;
        insert_into(loadavg).values(&v_nloadavg).execute(conn)?;
        insert_into(memory).values(&v_nmemory).execute(conn)?;
        insert_into(swap).values(&v_nswap).execute(conn)?;
        insert_into(disks).values(&v_ndisks).execute(conn)?;
        insert_into(ioblocks).values(&v_nioblocks).execute(conn)?;
        insert_into(ionets).values(&v_nionets).execute(conn)?;

        Ok(())
    }

    fn insert_one(conn: &mut ConnType, item: &HttpHost, huuid: &str) -> Result<(), ApiError> {
        // Insert Host data, if conflict, only update uptime
        insert_into(hosts)
            .values(HostDTO::cfrom(item, huuid))
            .on_conflict(uuid)
            .do_update()
            .set(uptime.eq(item.uptime))
            .execute(conn)?;

        // Only insert Option if they are present
        if let Some(value) = CpuStatsDTO::cfrom(item, huuid) {
            insert_into(cpustats).values(&value).execute(conn)?;
        }
        if let Some(value) = CpuTimesDTO::cfrom(item, huuid) {
            insert_into(cputimes).values(&value).execute(conn)?;
        }
        if let Some(value) = LoadAvgDTO::cfrom(item, huuid) {
            insert_into(loadavg).values(&value).execute(conn)?;
        }
        if let Some(value) = MemoryDTO::cfrom(item, huuid) {
            insert_into(memory).values(&value).execute(conn)?;
        }
        if let Some(value) = SwapDTO::cfrom(item, huuid) {
            insert_into(swap).values(&value).execute(conn)?;
        }
        if let Some(value) = DiskDTO::cfrom(item, huuid) {
            insert_into(disks).values(&value).execute(conn)?;
        }
        if let Some(value) = IoBlockDTO::cfrom(item, huuid) {
            insert_into(ioblocks).values(&value).execute(conn)?;
        }
        if let Some(value) = IoNetDTO::cfrom(item, huuid) {
            insert_into(ionets).values(&value).execute(conn)?;
        }

        Ok(())
    }

    pub fn list_hosts(conn: &mut ConnType, size: i64, page: i64) -> Result<Vec<Self>, ApiError> {
        Ok(dsl_host
            .limit(size)
            .offset(page * size)
            .order_by(hostname.asc())
            .load(conn)?)
    }

    pub fn get_from_uuids(
        conn: &mut ConnType,
        hosts_uuid: &[String],
    ) -> Result<Vec<Self>, ApiError> {
        Ok(dsl_host
            .filter(uuid.eq_any(hosts_uuid))
            .order_by(hostname.asc())
            .load(conn)?)
    }
}

impl<'a> BaseCrud<'a> for Host {
    type RetType = Host;

    type VecRetType = Vec<Self::RetType>;

    type TargetType = &'a str;

    type UuidType = &'a str;

    fn get(
        _conn: &mut ConnType,
        _uuid: Self::UuidType,
        _size: i64,
        _page: i64,
    ) -> Result<Self::VecRetType, ApiError> {
        todo!()
    }

    fn get_specific(
        conn: &mut ConnType,
        target_id: Self::TargetType,
    ) -> Result<Self::RetType, ApiError> {
        Ok(dsl_host.filter(uuid.eq(target_id)).first(conn)?)
    }
}

impl<'a> HostDTO<'a> {
    pub fn cfrom(item: &'a HttpHost, huuid: &'a str) -> Self {
        Self {
            system: &item.system,
            os_version: &item.os_version,
            hostname: &item.hostname,
            uptime: item.uptime,
            uuid: huuid,
            created_at: item.created_at,
        }
    }
}
