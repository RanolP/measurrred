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
                let taskbar_handle =
                    unsafe { FindWindowW("Shell_TrayWnd", PWSTR(null_mut())) }.ok()?;

                let bar_handle = unsafe {
                    FindWindowExW(taskbar_handle, HWND(0), "ReBarWindow32", PWSTR(null_mut()))
                }
                .ok()?;

                Ok(Taskbar(bar_handle))
            })
            .map(Clone::clone)
    }

    pub fn get() -> Option<&'static Taskbar> {
        TASKBAR.get()
    }

    pub fn rect(&self) -> eyre::Result<RECT> {
        let mut result = RECT::default();
        unsafe { GetWindowRect(self.0, &mut result) }.ok()?;

        Ok(result)
    }
}
