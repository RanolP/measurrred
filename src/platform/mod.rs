#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod platform;

pub use self::platform::*;
