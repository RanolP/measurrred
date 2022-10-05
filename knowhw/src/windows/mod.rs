#![cfg(target_os = "windows")]

pub use battery_report::{BatteryReport, BatteryReportError, BatteryReportQuery};
pub use global_memory_status::{GlobalMemoryStatus, GlobalMemoryStatusError, GlobalMemoryStatusQuery};

mod battery_report;
mod global_memory_status;
