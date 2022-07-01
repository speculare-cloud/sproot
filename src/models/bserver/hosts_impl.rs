use super::{
    CpuStatsDTO, CpuTimesDTO, DiskDTO, Host, HostDTO, HttpPostHost, IoBlockDTO, IoNetDTO,
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
use crate::ConnType;

use diesel::*;

impl Host {
    /// Insert the host data (update or create) (multiple value at once (Vec<HttpPostHost>))
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `items` - The Vec<HttpPostHost> we just got from the Post request (contains all our info)
    /// * `huuid` - The UUID of the host
    pub fn insert(
        conn: &mut ConnType,
        items: &[HttpPostHost],
        huuid: &str,
    ) -> Result<(), ApiError> {
        let len = items.len();
        // If there is only one item, it's faster to only insert one to avoid allocation of vector
        if len == 1 {
            return Self::insert_one(conn, &items[0], huuid);
        } else if len == 0 {
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
        // If we reached this point, everything went well so return an empty Closure
        Ok(())
    }

    /// Insert the host data (update or create) (one value of HttpPostHost at a time)
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `item` - The Vec<HttpPostHost>[0] we just got from the Post request (contains all our info)
    pub fn insert_one(
        conn: &mut ConnType,
        item: &HttpPostHost,
        huuid: &str,
    ) -> Result<(), ApiError> {
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
        // If we reached this point, everything went well so return an empty Closure
        Ok(())
    }

    /// Return a Vector of Host
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn list_hosts(conn: &mut ConnType, size: i64, page: i64) -> Result<Vec<Self>, ApiError> {
        Ok(dsl_host
            .limit(size)
            .offset(page * size)
            .order_by(hostname.asc())
            .load(conn)?)
    }

    /// Return a Vector of Host
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `hosts_uuid` - The uuids of the hosts you want to get info
    pub fn get_from_uuids(
        conn: &mut ConnType,
        hosts_uuid: &[String],
    ) -> Result<Vec<Self>, ApiError> {
        Ok(dsl_host
            .filter(uuid.eq_any(hosts_uuid))
            .order_by(hostname.asc())
            .load(conn)?)
    }

    /// Return a Host from his UUID
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `huuid` - The uuids of the hosts you want to get info
    pub fn get_from_uuid(conn: &mut ConnType, huuid: &str) -> Result<Self, ApiError> {
        Ok(dsl_host.filter(uuid.eq(huuid)).first(conn)?)
    }
}

impl<'a> HostDTO<'a> {
    pub fn cfrom(item: &'a HttpPostHost, huuid: &'a str) -> Self {
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
