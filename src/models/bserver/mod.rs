mod cpustats;
mod cpustats_impl;
pub use cpustats::*;
pub use cpustats_impl::*;

mod cputimes;
mod cputimes_impl;
pub use cputimes::*;
pub use cputimes_impl::*;

mod disk;
mod disk_impl;
pub use disk::*;
pub use disk_impl::*;

mod hosts;
mod hosts_impl;
pub use hosts::*;
pub use hosts_impl::*;

mod ionet;
mod ionet_impl;
pub use ionet::*;
pub use ionet_impl::*;

mod ioblock;
mod ioblock_impl;
pub use ioblock::*;
pub use ioblock_impl::*;

mod loadavg;
mod loadavg_impl;
pub use loadavg::*;
pub use loadavg_impl::*;

mod memory;
mod memory_impl;
pub use memory::*;
pub use memory_impl::*;

mod swap;
mod swap_impl;
pub use swap::*;

use crate::{ApiError, ConnType};

pub trait BaseMetrics {
    type VecReturn;
    type VecRawReturn;

    fn get(
        conn: &mut ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Self::VecReturn, ApiError>;

    fn get_dated(
        conn: &mut ConnType,
        uuid: &str,
        min_date: chrono::NaiveDateTime,
        max_date: chrono::NaiveDateTime,
    ) -> Result<Self::VecRawReturn, ApiError>;
}

pub trait ExtMetrics {
    fn count_unique(conn: &mut ConnType, uuid: &str, size: i64) -> Result<i64, ApiError>;
}

pub trait CFrom<T>: Sized {
    type RET;
    type UUID;

    fn cfrom(_: T, huuid: Self::UUID) -> Option<Self::RET>;
}
