use std::{collections::HashMap, ptr::null_mut};

use windows::Win32::{
    Foundation::{GetLastError, PWSTR},
    System::{
        Performance::{
            PdhAddEnglishCounterW, PdhCollectQueryData, PdhGetFormattedCounterValue, PdhOpenQueryW,
            PDH_CALC_NEGATIVE_DENOMINATOR, PDH_FMT_COUNTERVALUE, PDH_FMT_DOUBLE, PDH_FMT_LONG,
            PDH_INVALID_DATA,
        },
        SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX},
    },
};

use super::{Data, DataHandle, DataSource, PreferredDataFormat};

pub struct GlobalMemoryStatusDataSource;

impl DataSource for GlobalMemoryStatusDataSource {
    fn name(&self) -> String {
        "global-memory-status".to_string()
    }

    fn update(&self) -> eyre::Result<()> {
        Ok(())
    }

    fn query(
        &mut self,
        query: String,
        preferred_format: PreferredDataFormat,
    ) -> eyre::Result<DataHandle> {
        Ok(DataHandle(Box::new(move || {
            let query: &str = &query;

            let mut mem = MEMORYSTATUSEX::default();
            mem.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;

            let result = unsafe { GlobalMemoryStatusEx(&mut mem) };

            if result.0 == 0 {
                Err(::windows::core::Error::from_win32())?;
            }

            let result = match query {
                "dwMemoryLoad" => mem.dwMemoryLoad as u64,
                "ullTotalPhys" => mem.ullTotalPhys,
                "ullAvailPhys" => mem.ullAvailPhys,
                "ullUsedPhys" => mem.ullTotalPhys - mem.ullAvailPhys,
                "ullTotalPageFile" => mem.ullTotalPageFile,
                "ullAvailPageFile" => mem.ullAvailPageFile,
                "ullTotalVirtual" => mem.ullTotalVirtual,
                "ullAvailVirtual" => mem.ullAvailVirtual,
                "ullAvailExtendedVirtual" => mem.ullAvailExtendedVirtual,
                _ => eyre::bail!("Unknown query: {}", query),
            };

            let data = match preferred_format {
                PreferredDataFormat::Int => Data::Int(result as i64),
                PreferredDataFormat::Float => Data::Float(result as f64),
                PreferredDataFormat::Boolean => Data::Int(result as i64),
            };

            Ok(data)
        })))
    }
}
