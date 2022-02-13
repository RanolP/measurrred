#![cfg(target_os = "windows")]

use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

use crate::{
    data_source::DataSource,
    system::{Data, DataFormat, DataHandle},
};

pub struct GlobalMemoryStatusDataSource;

impl DataSource for GlobalMemoryStatusDataSource {
    fn name(&self) -> &'static str {
        "windows/global-memory-status"
    }

    fn update(&self) -> eyre::Result<()> {
        Ok(())
    }

    fn query(&mut self, query: String, _preferred_format: DataFormat) -> eyre::Result<DataHandle> {
        Ok(DataHandle(Box::new(move || {
            let query: &str = &query;

            let mut mem = MEMORYSTATUSEX::default();
            mem.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;

            let result = unsafe { GlobalMemoryStatusEx(&mut mem) };

            if result.0 == 0 {
                Err(::windows::core::Error::from_win32())?;
            }

            let data = match query {
                "dwMemoryLoad" => Data::I64(mem.dwMemoryLoad as i64),
                "ullTotalPhys" => Data::I64(mem.ullTotalPhys as i64),
                "ullAvailPhys" => Data::I64(mem.ullAvailPhys as i64),
                "ullUsedPhys" => Data::I64((mem.ullTotalPhys - mem.ullAvailPhys) as i64),
                "ullTotalPageFile" => Data::I64(mem.ullTotalPageFile as i64),
                "ullAvailPageFile" => Data::I64(mem.ullAvailPageFile as i64),
                "ullTotalVirtual" => Data::I64(mem.ullTotalVirtual as i64),
                "ullAvailVirtual" => Data::I64(mem.ullAvailVirtual as i64),
                "ullAvailExtendedVirtual" => Data::I64(mem.ullAvailExtendedVirtual as i64),
                "dMemoryLoad" => Data::F64(
                    (mem.ullTotalPhys - mem.ullAvailPhys) as f64 / mem.ullTotalPhys as f64 * 100.0,
                ),
                _ => eyre::bail!("Unknown query: {}", query),
            };

            Ok(data)
        })))
    }
}
