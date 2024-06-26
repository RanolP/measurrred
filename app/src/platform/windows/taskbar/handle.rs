use std::{ffi::OsString, os::windows::prelude::OsStringExt, ptr::null_mut};

use tracing_unwrap::OptionExt;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{BOOL, HWND, LPARAM, RECT},
        UI::WindowsAndMessaging::{EnumWindows, FindWindowExW, GetClassNameW, GetWindowRect},
    },
};

use crate::{platform::monitor::MonitorHandle, system::Rect};

#[derive(Clone, Debug)]
pub struct TaskbarHandle {
    hwnd: HWND,
    rebar_hwnd: HWND,
    tray_notify_hwnd: HWND,
    monitor: MonitorHandle,
}

impl TaskbarHandle {
    pub fn collect() -> windows::core::Result<Vec<TaskbarHandle>> {
        type ResultVec = Vec<HWND>;
        let mut found_windows = ResultVec::new();
        unsafe {
            fn is_taskbar_handle(hwnd: HWND) -> bool {
                const CLASS_NAME: &str = "Shell_TrayWnd";
                const CLASS_NAME_LENGTH: usize = CLASS_NAME.len() + 1;

                let mut name = vec![0u16; CLASS_NAME_LENGTH];

                let len = unsafe { GetClassNameW(hwnd, &mut name) as usize };

                if len == 0 {
                    return false;
                }

                let name = OsString::from_wide(&name[..len]);

                CLASS_NAME == name
            }
            unsafe extern "system" fn filter(hwnd: HWND, lparam: LPARAM) -> BOOL {
                let found_windows = (lparam.0 as *mut ResultVec).as_mut().unwrap_or_log();

                if is_taskbar_handle(hwnd) {
                    found_windows.push(hwnd);
                }

                BOOL::from(true)
            }
            EnumWindows(
                Some(filter),
                LPARAM(&mut found_windows as *mut ResultVec as _),
            )
        }
        .ok()?;

        found_windows
            .into_iter()
            .map(|hwnd| {
                let rebar_hwnd =
                    unsafe { FindWindowExW(hwnd, HWND(0), "ReBarWindow32", PCWSTR(null_mut())) };
                if rebar_hwnd.0 == 0 {
                    Err(windows::core::Error::from_win32())?
                }

                let tray_notify_hwnd =
                    unsafe { FindWindowExW(hwnd, HWND(0), "TrayNotifyWnd", PCWSTR(null_mut())) };
                if tray_notify_hwnd.0 == 0 {
                    Err(windows::core::Error::from_win32())?
                }

                Ok(TaskbarHandle {
                    hwnd,
                    rebar_hwnd,
                    tray_notify_hwnd,
                    // We knew that the taskbar is belong to the monitor.
                    // It cannot be moved out to other monitors, isn't it?
                    monitor: MonitorHandle::from_hwnd(hwnd),
                })
            })
            .collect()
    }

    pub fn hwnd(&self) -> &HWND {
        &self.hwnd
    }

    pub fn rebar_hwnd(&self) -> &HWND {
        &self.rebar_hwnd
    }

    pub fn rect(&self) -> windows::core::Result<Rect> {
        let mut shell = RECT::default();
        unsafe { GetWindowRect(self.hwnd, &mut shell) }.ok()?;

        let mut rebar = RECT::default();
        unsafe { GetWindowRect(self.rebar_hwnd, &mut rebar) }.ok()?;

        let mut tray = RECT::default();
        unsafe { GetWindowRect(self.tray_notify_hwnd, &mut tray) }.ok()?;

        Ok(Rect::from_xywh(
            shell.left,
            rebar.top,
            tray.left - shell.left,
            shell.bottom,
        ))
    }

    pub fn monitor(&self) -> &MonitorHandle {
        &self.monitor
    }
}
