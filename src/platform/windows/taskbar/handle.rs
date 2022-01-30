use std::{ffi::OsString, os::windows::prelude::OsStringExt};

use tracing_unwrap::OptionExt;
use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM, PWSTR, RECT},
    UI::WindowsAndMessaging::{EnumWindows, GetClassNameW, GetWindowRect},
};

use crate::{platform::monitor::MonitorHandle, system::Rect};

#[derive(Clone, Debug)]
pub struct TaskbarHandle {
    pub(super) hwnd: HWND,
}

impl TaskbarHandle {
    pub fn collect() -> windows::core::Result<Vec<TaskbarHandle>> {
        type ResultVec = Vec<TaskbarHandle>;
        let mut found_windows = ResultVec::new();
        unsafe {
            fn is_taskbar_handle(hwnd: HWND) -> bool {
                const CLASS_NAME: &str = "Shell_TrayWnd";
                const CLASS_NAME_LENGTH: usize = CLASS_NAME.len() + 1;

                let mut name = vec![0u16; CLASS_NAME_LENGTH];

                let len = unsafe {
                    GetClassNameW(
                        hwnd,
                        PWSTR(name.as_mut_ptr() as _),
                        CLASS_NAME_LENGTH as i32,
                    ) as usize
                };

                if len == 0 {
                    return false;
                }

                let name = OsString::from_wide(&name[..len]);

                CLASS_NAME == name
            }
            unsafe extern "system" fn filter(hwnd: HWND, lparam: LPARAM) -> BOOL {
                let found_windows = (lparam.0 as *mut ResultVec).as_mut().unwrap_or_log();

                if is_taskbar_handle(hwnd) {
                    found_windows.push(TaskbarHandle { hwnd });
                }

                BOOL::from(true)
            }
            EnumWindows(
                Some(filter),
                LPARAM(&mut found_windows as *mut ResultVec as _),
            )
        }
        .ok()?;

        Ok(found_windows)
    }

    fn evaluate_rect(&self, respect_dpi: bool) -> windows::core::Result<Rect> {
        let mut result = RECT::default();
        unsafe { GetWindowRect(self.hwnd, &mut result) }.ok()?;

        if respect_dpi {
            let dpi = self.monitor().get_dpi()?;

            result.left = (result.left as f64 * dpi as f64 / 96.0) as i32;
            result.right = (result.right as f64 * dpi as f64 / 96.0) as i32;
            result.top = (result.top as f64 * dpi as f64 / 96.0) as i32;
            result.bottom = (result.bottom as f64 * dpi as f64 / 96.0) as i32;
        }

        Ok(result.into())
    }

    pub fn rect(&self) -> windows::core::Result<Rect> {
        self.evaluate_rect(true)
    }

    pub fn rect_raw(&self) -> windows::core::Result<Rect> {
        self.evaluate_rect(false)
    }

    pub fn monitor(&self) -> MonitorHandle {
        MonitorHandle::from_hwnd(self.hwnd)
    }
}
