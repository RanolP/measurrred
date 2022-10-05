use std::str::FromStr;

use declarrred::rt::{Data, DataFormat};
use strum::EnumString;
use thiserror::Error;
use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

use crate::Knowhw;

pub struct GlobalMemoryStatus;

#[derive(EnumString)]
pub enum GlobalMemoryStatusQuery {
    #[strum(serialize = "dwMemoryLoad")]
    MemoryLoad,

    #[strum(serialize = "ullTotalPhys")]
    TotalPhysical,

    #[strum(serialize = "ullAvailPhys")]
    AvailablePhysical,

    #[strum(serialize = "ullTotalPageFile")]
    TotalPageFile,

    #[strum(serialize = "ullAvailPageFile")]
    AvailablePageFile,

    #[strum(serialize = "ullTotalVirtual")]
    TotalVirtual,

    #[strum(serialize = "ullAvailVirtual")]
    AvailableVirtual,

    #[strum(serialize = "ullAvailExtendedVirtual")]
    AvailableExtendedVirtual,

    #[strum(serialize = "ullUsedPhys")]
    UsedPhysical,

    #[strum(serialize = "dMemoryLoad")]
    MemoryLoadPercent,
}

#[derive(Debug, Error)]
pub enum GlobalMemoryStatusError {
    #[error("Failed to parse query: {0}")]
    InvalidQuery(#[from] strum::ParseError),

    #[error("Win32 error: {0}")]
    WindowsError(#[from] windows::core::Error),
}

impl Knowhw for GlobalMemoryStatus {
    type Error = GlobalMemoryStatusError;

    fn query(&mut self, query: &str, _preferred_format: &DataFormat) -> Result<Data, Self::Error> {
        let query = GlobalMemoryStatusQuery::from_str(query)?;

        let mut mem = MEMORYSTATUSEX::default();
        mem.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;

        let result = unsafe { GlobalMemoryStatusEx(&mut mem) };

        if result.0 == 0 {
            Err(::windows::core::Error::from_win32())?;
        }

        let data = match query {
            GlobalMemoryStatusQuery::MemoryLoad => Data::U32(mem.dwMemoryLoad),
            GlobalMemoryStatusQuery::TotalPhysical => Data::U64(mem.ullTotalPhys),
            GlobalMemoryStatusQuery::AvailablePhysical => Data::U64(mem.ullAvailPhys),
            GlobalMemoryStatusQuery::UsedPhysical => Data::U64(mem.ullTotalPhys - mem.ullAvailPhys),
            GlobalMemoryStatusQuery::TotalPageFile => Data::U64(mem.ullTotalPageFile),
            GlobalMemoryStatusQuery::AvailablePageFile => Data::U64(mem.ullAvailPageFile),
            GlobalMemoryStatusQuery::TotalVirtual => Data::U64(mem.ullTotalVirtual),
            GlobalMemoryStatusQuery::AvailableVirtual => Data::U64(mem.ullAvailVirtual),
            GlobalMemoryStatusQuery::AvailableExtendedVirtual => {
                Data::U64(mem.ullAvailExtendedVirtual)
            }
            GlobalMemoryStatusQuery::MemoryLoadPercent => Data::F64(
                (mem.ullTotalPhys - mem.ullAvailPhys) as f64 / mem.ullTotalPhys as f64 * 100.0,
            ),
        };

        Ok(data)
    }
}
