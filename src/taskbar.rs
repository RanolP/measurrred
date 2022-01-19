use std::ptr::null_mut;

use windows::Win32::{
    Foundation::{HWND, PWSTR, RECT},
    UI::WindowsAndMessaging::{FindWindowExW, FindWindowW, GetWindowRect},
};

#[derive(Clone)]
pub struct Taskbar(pub(super) HWND);

impl Taskbar {
    pub fn new() -> eyre::Result<Taskbar> {
        let taskbar_handle = unsafe { FindWindowW("Shell_TrayWnd", PWSTR(null_mut())) }.ok()?;

        let bar_handle =
            unsafe { FindWindowExW(taskbar_handle, HWND(0), "ReBarWindow32", PWSTR(null_mut())) }
                .ok()?;

        Ok(Taskbar(bar_handle))
    }

    pub fn rect(&self) -> eyre::Result<RECT> {
        let mut result = RECT::default();
        unsafe { GetWindowRect(self.0, &mut result) }.ok()?;

        Ok(result)
    }
}
