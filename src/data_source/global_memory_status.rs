#![cfg(target_os = "windows")]

use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

use crate::system::{Data, DataFormat, DataHandle};

use super::DataSource;

pub struct GlobalMemoryStatusDataSource;

impl DataSource for GlobalMemoryStatusDataSource {
    fn name(&self) -> &'static str {
        "global-memory-status"
    }

    fn update(&self) -> eyre::Result<()> {
        Ok(())
    }

    fn query(&mut self, query: String, preferred_format: DataFormat) -> eyre::Result<DataHandle> {
        Ok(DataHandle(Box::new(move || {
            let query: &str = &query;

            let mut mem = MEMORYSTATUSEX::default();
            mem.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;

            let result = unsafe { GlobalMemoryStatusEx(&mut mem) };

            if result.0 == 0 {
                Err(::windows::core::Error::from_win32())?;
            }

            let result = match query {
                "dwMemoryLoad" => mem.dwMemoryLoad as f64,
                "ullTotalPhys" => mem.ullTotalPhys as f64,
                "ullAvailPhys" => mem.ullAvailPhys as f64,
                "ullUsedPhys" => (mem.ullTotalPhys - mem.ullAvailPhys) as f64,
                "ullTotalPageFile" => mem.ullTotalPageFile as f64,
                "ullAvailPageFile" => mem.ullAvailPageFile as f64,
                "ullTotalVirtual" => mem.ullTotalVirtual as f64,
                "ullAvailVirtual" => mem.ullAvailVirtual as f64,
                "ullAvailExtendedVirtual" => mem.ullAvailExtendedVirtual as f64,
                "dMemoryLoad" => {
                    (mem.ullTotalPhys - mem.ullAvailPhys) as f64 / mem.ullTotalPhys as f64 * 100.0
                }
                _ => eyre::bail!("Unknown query: {}", query),
            };

            let data = match &preferred_format {
                f @ DataFormat::String | f @ DataFormat::Bool => {
                    eyre::bail!("Unsupported format: {:?}", f)
                }
                DataFormat::I32 | DataFormat::I64 | DataFormat::Int => Data::I64(result as i64),
                DataFormat::F64 | DataFormat::Float => Data::F64(result),
            };

            Ok(data)
        })))
    }
}
