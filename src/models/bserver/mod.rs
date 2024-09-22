mod cpustats;
mod cpustats_impl;
pub use cpustats::*;

mod cputimes;
mod cputimes_impl;
pub use cputimes::*;

mod disk;
mod disk_impl;
pub use disk::*;

mod hosts;
mod hosts_impl;
pub use hosts::*;

mod ionet;
mod ionet_impl;
pub use ionet::*;

mod ioblock;
mod ioblock_impl;
pub use ioblock::*;

mod loadavg;
mod loadavg_impl;
pub use loadavg::*;

mod memory;
mod memory_impl;
pub use memory::*;

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

pub trait CFrom<T>: Sized {
    type RET;
    type UUID;

    fn cfrom(_: T, huuid: Self::UUID) -> Option<Self::RET>;
}
