#![cfg(target_os = "windows")]

pub use global_memory_status::GlobalMemoryStatusDataSource;
pub use pdh::PdhDataSource;

mod global_memory_status;
mod pdh;
