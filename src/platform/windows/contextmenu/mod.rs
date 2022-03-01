use std::ptr::null_mut;

use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{
        AppendMenuW, CreatePopupMenu, SetForegroundWindow, TrackPopupMenu, HMENU, MF_STRING,
        TPM_LEFTALIGN, TPM_LEFTBUTTON, WM_USER,
    },
};

pub struct ContextMenu {
    handler_list: Vec<Box<dyn FnMut() -> windows::core::Result<()> + Send + Sync>>,
    handle: HMENU,
}

impl ContextMenu {
    pub fn new(
        item_list: Vec<(
            String,
            Box<dyn FnMut() -> windows::core::Result<()> + Send + Sync>,
        )>,
    ) -> windows::core::Result<Self> {
        let handle = unsafe { CreatePopupMenu() }.ok()?;

        for (idx, (name, _)) in item_list.iter().enumerate() {
            unsafe { AppendMenuW(handle, MF_STRING, WM_USER as usize + idx, name.clone()) }.ok()?;
        }

        let menu = ContextMenu {
            handler_list: item_list.into_iter().map(|(_, handler)| handler).collect(),
            handle,
        };

        Ok(menu)
    }

    pub fn show(&self, hwnd: HWND, x: i32, y: i32) -> eyre::Result<()> {
        unsafe { SetForegroundWindow(hwnd) };
        unsafe {
            TrackPopupMenu(
                self.handle,
                TPM_LEFTALIGN | TPM_LEFTBUTTON,
                x,
                y,
                0,
                hwnd,
                null_mut(),
            )
        }
        .ok()?;
        unsafe { SetForegroundWindow(hwnd) };
        Ok(())
    }

    pub fn handle_message(&mut self, message: usize) -> windows::core::Result<()> {
        dbg!(message);
        (self.handler_list[(message - WM_USER as usize)])()
    }

    pub fn hide(&self) {}
}
