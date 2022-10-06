#![cfg(target_os = "windows")]

pub use battery_report::{BatteryReport, BatteryReportError, BatteryReportQuery};
pub use global_memory_status::{
    GlobalMemoryStatus, GlobalMemoryStatusError, GlobalMemoryStatusQuery,
};
pub use pdh::{Pdh, PdhError};

mod battery_report;
mod global_memory_status;
mod pdh;
