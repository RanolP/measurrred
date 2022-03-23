use std::ptr::null_mut;

use thiserror::Error;
use tracing_unwrap::ResultExt;
use windows::{Win32::{
    Foundation::{HWND, LPARAM, LRESULT, POINT, WPARAM},
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        Shell::{
            Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE,
            NOTIFYICONDATAW, NOTIFYICONDATAW_0, NOTIFYICON_VERSION_4,
        },
        WindowsAndMessaging::{
            DefWindowProcW, DestroyWindow, GetCursorPos, LoadIconW, WM_COMMAND, WM_LBUTTONDBLCLK,
            WM_RBUTTONDOWN, WM_USER,
        },
    },
}, core::PCWSTR};

use crate::platform::contextmenu::ContextMenu;

#[derive(Clone)]
pub struct TrayIcon {
    data: NOTIFYICONDATAW,
}

pub struct ActualTrayIcon {
    menu: ContextMenu,
}

#[derive(Error, Debug)]
pub enum TrayIconError {
    #[error("Windows API call failed: 0x{:x}", .0.code().0)]
    Windows(#[from] windows::core::Error),
    #[error("Mutex lock poisoned")]
    MutexLockPoisoned,
}

const IDI_TRAYICON: PCWSTR = PCWSTR(1101 as _);

impl TrayIcon {
    const UID: u32 = 1000;
    const MESSAGE_ID: u32 = WM_USER + 1;

    pub fn new(main_window: HWND) -> Result<(TrayIcon, ActualTrayIcon), TrayIconError> {
        let instance = unsafe { GetModuleHandleW(PCWSTR(null_mut())) }.ok()?;

        let icon = unsafe { LoadIconW(instance, IDI_TRAYICON) }.ok()?;

        let mut tip = [0u16; 128];

        for (i, c) in "measurrred".encode_utf16().enumerate() {
            tip[i] = c;
        }

        let mut data = NOTIFYICONDATAW {
            hWnd: main_window,
            uID: TrayIcon::UID,
            uFlags: NIF_ICON | NIF_TIP | NIF_MESSAGE,
            uCallbackMessage: TrayIcon::MESSAGE_ID,
            hIcon: icon,
            szTip: tip,
            Anonymous: NOTIFYICONDATAW_0 {
                uVersion: NOTIFYICON_VERSION_4,
            },
            ..Default::default()
        };
        data.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;

        let actual = ActualTrayIcon {
            menu: ContextMenu::new(vec![
                (
                    "Settings".to_string(),
                    Box::new(|| {
                        dbg!("세팅 열어주세요");
                        Ok(())
                    }),
                ),
                (
                    "Quit".to_string(),
                    Box::new(move || {
                        dbg!("죽어주세요");
                        unsafe { DestroyWindow(main_window) }.ok()
                    }),
                ),
            ])?,
        };

        Ok((TrayIcon { data }, actual))
    }

    pub fn add(&self) -> Result<(), TrayIconError> {
        unsafe { Shell_NotifyIconW(NIM_ADD, &self.data) }.ok()?;
        Ok(())
    }

    pub fn remove(&self) -> Result<(), TrayIconError> {
        unsafe { Shell_NotifyIconW(NIM_DELETE, &self.data) }.ok()?;
        Ok(())
    }
}

pub enum HandleResult {
    Ok(LRESULT),
    MessageMismatch,
    ContextMenuAction,
}

impl ActualTrayIcon {
    pub fn can_accept(msg: u32) -> bool {
        matches!(msg, TrayIcon::MESSAGE_ID | WM_COMMAND)
    }

    pub fn handle(&self, hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> HandleResult {
        match msg {
            TrayIcon::MESSAGE_ID => (),
            WM_COMMAND => return HandleResult::ContextMenuAction,
            _ => return HandleResult::MessageMismatch,
        }

        let res = match lparam.0 as u32 {
            WM_RBUTTONDOWN => {
                let mut pos = POINT::default();
                unsafe { GetCursorPos(&mut pos) }.ok().unwrap();

                self.menu.show(hwnd, pos.x, pos.y).unwrap_or_log();
                LRESULT(0)
            }
            WM_LBUTTONDBLCLK => LRESULT(0),
            _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
        };

        HandleResult::Ok(res)
    }

    pub fn handle_context_menu(
        &mut self,
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<LRESULT> {
        if msg != WM_COMMAND {
            return None;
        }

        let res = if wparam.0 >= WM_USER as usize {
            self.menu.handle_message(wparam.0).unwrap_or_log();
            LRESULT(0)
        } else {
            unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
        };

        Some(res)
    }
}
