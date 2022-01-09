use std::ptr::null_mut;

use once_cell::sync::OnceCell;
use windows::Win32::{
    Foundation::{GetLastError, HWND, PWSTR, RECT},
    UI::WindowsAndMessaging::{FindWindowExW, FindWindowW, GetWindowRect},
};

static TASKBAR: OnceCell<Taskbar> = OnceCell::new();

#[derive(Clone)]
pub struct Taskbar(pub(super) HWND);

impl Taskbar {
    pub fn try_initialize() -> eyre::Result<Taskbar> {
        TASKBAR
            .get_or_try_init(|| {
                let taskbar_handle = unsafe { FindWindowW("Shell_TrayWnd", PWSTR(null_mut())) };
                if taskbar_handle == 0 {
                    eyre::bail!("Failed to find Shell_TrayWnd: Error Code 0x{:x}", unsafe {
                        GetLastError()
                    });
                }

                let bar_handle =
                    unsafe { FindWindowExW(taskbar_handle, 0, "ReBarWindow32", PWSTR(null_mut())) };
                if bar_handle == 0 {
                    eyre::bail!(
                        "Failed to find ReBarWindow32 in Shell_TrayWnd: Error Code 0x{:x}",
                        unsafe { GetLastError() }
                    );
                }

                Ok(Taskbar(bar_handle))
            })
            .map(Clone::clone)
    }

    pub fn rect(&self) -> eyre::Result<RECT> {
        let mut result = RECT::default();
        let failed = unsafe { GetWindowRect(self.0, &mut result).0 == 0 };
        if failed {
            eyre::bail!("Failed to get Taskbar RECT: Error Code 0x{:x}", unsafe {
                GetLastError()
            });
        }

        Ok(result)
    }
}
