#![cfg(target_os = "windows")]

pub use battery_report::BatteryReportDataSource;
pub use global_memory_status::GlobalMemoryStatusDataSource;
pub use pdh::PdhDataSource;

mod battery_report;
mod global_memory_status;
mod pdh;
