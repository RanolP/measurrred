use std::{collections::HashMap, ptr::null_mut, sync::RwLock};

use once_cell::sync::Lazy;
use thiserror::Error;
use tracing_unwrap::ResultExt;
use windows::Win32::{
    Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, PWSTR, WPARAM},
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        Shell::{
            Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE,
            NOTIFYICONDATAW, NOTIFYICONDATAW_0, NOTIFYICON_VERSION_4,
        },
        WindowsAndMessaging::{
            DefWindowProcW, DestroyWindow, LoadIconW, PostQuitMessage, IDI_APPLICATION,
            WM_LBUTTONDBLCLK, WM_RBUTTONDOWN, WM_USER,
        },
    },
};

static OVERLAY_INSTANCES: Lazy<RwLock<HashMap<isize, ActualTrayIcon>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Clone)]
pub struct TrayIcon {
    data: NOTIFYICONDATAW,
}

pub struct ActualTrayIcon {}

#[derive(Error, Debug)]
pub enum TrayIconError {
    #[error("Windows API call failed: 0x{:x}", .0.code().0)]
    Windows(#[from] windows::core::Error),
    #[error("Mutex lock poisoned")]
    MutexLockPoisoned,
}

const IDI_TRAYICON: PWSTR = PWSTR(1101 as _);

impl TrayIcon {
    const UID: u32 = 1000;
    const MESSAGE_ID: u32 = WM_USER + 1;

    pub fn new(main_window: HWND) -> Result<Self, TrayIconError> {
        let instance = unsafe { GetModuleHandleW(PWSTR(null_mut())) }.ok()?;

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

        let actual = ActualTrayIcon {};

        OVERLAY_INSTANCES
            .write()
            .map_err(|_| TrayIconError::MutexLockPoisoned)?
            .insert(0, actual);

        Ok(TrayIcon { data })
    }

    pub fn add(&self) -> Result<(), TrayIconError> {
        unsafe { Shell_NotifyIconW(NIM_ADD, &self.data) }.ok()?;
        Ok(())
    }

    pub fn remove(&self) -> Result<(), TrayIconError> {
        unsafe { Shell_NotifyIconW(NIM_DELETE, &self.data) }.ok()?;
        Ok(())
    }

    pub fn handle(&self, hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> Option<LRESULT> {
        if msg != TrayIcon::MESSAGE_ID {
            return None;
        }

        let res = match lparam.0 as u32 {
            WM_RBUTTONDOWN => {
                dbg!("right button down");
                // Treat this as a quit
                unsafe { DestroyWindow(hwnd) }.ok().unwrap_or_log();
                LRESULT(0)
            }
            WM_LBUTTONDBLCLK => {
                dbg!("left button double click");
                LRESULT(0)
            }
            _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
        };

        Some(res)
    }
}
