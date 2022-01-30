use std::{collections::HashMap, ptr::null_mut, sync::RwLock};

use once_cell::sync::Lazy;
use thiserror::Error;
use tiny_skia::Pixmap;
use tracing_unwrap::{OptionExt, ResultExt};
use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, PWSTR, RECT, WPARAM},
    Graphics::Gdi::{
        BeginPaint, BitBlt, CreateBitmap, CreateCompatibleDC, CreateSolidBrush, DeleteDC,
        DeleteObject, EndPaint, FillRect, RedrawWindow, SelectObject, SetBkMode, HRGN, PAINTSTRUCT,
        RDW_INVALIDATE, RDW_UPDATENOW, SRCPAINT, TRANSPARENT,
    },
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, DispatchMessageW, GetClientRect, GetMessageW, MoveWindow,
        PostQuitMessage, RegisterClassW, SetLayeredWindowAttributes, ShowWindow, TranslateMessage,
        CS_HREDRAW, CS_VREDRAW, HMENU, LWA_COLORKEY, MSG, SW_SHOW, WM_DESTROY, WM_DPICHANGED,
        WM_PAINT, WNDCLASSW, WS_CHILD, WS_EX_LAYERED, WS_EX_TOPMOST, WS_VISIBLE,
    },
};

use crate::{config::MeasurrredConfig, platform::dpi::become_dpi_aware};

use super::TaskbarHandle;

static OVERLAY_INSTANCES: Lazy<RwLock<HashMap<isize, ActualTaskbarOverlay>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Clone)]
pub struct TaskbarOverlay {
    pub target: TaskbarHandle,
    hwnd: HWND,
}

pub struct ActualTaskbarOverlay {
    hwnd: HWND,
    target: TaskbarHandle,
    pixmap: Option<Pixmap>,
    background_color: u32,
}

#[derive(Error, Debug)]
pub enum TaskbarOverlayError {
    #[error("Windows API call failed: 0x{:x}", .0.code().0)]
    Windows(#[from] windows::core::Error),
    #[error("Mutex lock poisoned")]
    MutexLockPoisoned,
}

impl TaskbarOverlay {
    const CLASS_NAME_STR: &'static str = "MeasurredTaskbar";
    const CLASS_NAME: PWSTR = PWSTR(TaskbarOverlay::CLASS_NAME_STR.as_ptr() as _);

    pub fn new(target: TaskbarHandle) -> Result<Self, TaskbarOverlayError> {
        become_dpi_aware()?;

        let instance = unsafe { GetModuleHandleW(PWSTR(null_mut())) }.ok()?;

        let class = WNDCLASSW {
            lpfnWndProc: Some(wndproc),
            hInstance: instance,
            lpszClassName: TaskbarOverlay::CLASS_NAME,
            style: CS_HREDRAW | CS_VREDRAW,
            ..Default::default()
        };

        let class_id = unsafe { RegisterClassW(&class) };
        if class_id == 0 {
            Err(::windows::core::Error::from_win32())?;
        }

        let hwnd = unsafe {
            CreateWindowExW(
                WS_EX_TOPMOST | WS_EX_LAYERED,
                TaskbarOverlay::CLASS_NAME,
                "measurrred",
                WS_VISIBLE | WS_CHILD,
                0,
                0,
                0,
                0,
                target.hwnd,
                HMENU(0),
                instance,
                null_mut(),
            )
        }
        .ok()?;

        let overlay = ActualTaskbarOverlay {
            hwnd: hwnd.clone(),
            target: target.clone(),
            background_color: 0,
            pixmap: None,
        };

        overlay.update_layout()?;

        OVERLAY_INSTANCES
            .write()
            .map_err(|_| TaskbarOverlayError::MutexLockPoisoned)?
            .insert(hwnd.0, overlay);

        Ok(TaskbarOverlay { target, hwnd })
    }

    pub fn accept_config(&mut self, config: &MeasurrredConfig) -> Result<(), TaskbarOverlayError> {
        let mut map = OVERLAY_INSTANCES
            .write()
            .map_err(|_| TaskbarOverlayError::MutexLockPoisoned)?;
        let mut actual_self = map.get_mut(&self.hwnd.0).unwrap_or_log();

        actual_self.background_color = config.background_color.to_windows_color();

        unsafe {
            SetLayeredWindowAttributes(self.hwnd, actual_self.background_color, 0, LWA_COLORKEY)
        }
        .ok()?;

        Ok(())
    }

    pub fn accept_pixmap(&mut self, pixmap: Pixmap) -> Result<(), TaskbarOverlayError> {
        let mut map = OVERLAY_INSTANCES
            .write()
            .map_err(|_| TaskbarOverlayError::MutexLockPoisoned)?;
        let mut actual_self = map.get_mut(&self.hwnd.0).unwrap_or_log();

        actual_self.pixmap = Some(pixmap);

        Ok(())
    }

    pub fn show(&self) -> bool {
        unsafe { ShowWindow(self.hwnd, SW_SHOW).as_bool() }
    }

    pub fn redraw(&self) -> eyre::Result<()> {
        let fail = unsafe {
            RedrawWindow(
                self.hwnd,
                null_mut(),
                HRGN(0),
                RDW_INVALIDATE | RDW_UPDATENOW,
            )
            .0 == 0
        };
        if fail {
            Err(windows::core::Error::from_win32())?
        }

        Ok(())
    }

    pub fn begin_event_loop(&self) -> eyre::Result<()> {
        let mut message = MSG::default();
        let mut message_status: i32;
        while unsafe {
            message_status = GetMessageW(&mut message, HWND(0), 0, 0).0;
            message_status != 0
        } {
            if message_status == -1 {
                Err(windows::core::Error::from_win32())?
            }
            unsafe {
                TranslateMessage(&message);
                // its return value, LRESULT, is generally ignored
                DispatchMessageW(&mut message);
            }
        }
        Ok(())
    }

    pub fn zoom(&self) -> eyre::Result<f32> {
        Ok(self.target.monitor().get_dpi()? as f32 / 96.0)
    }
}

impl ActualTaskbarOverlay {
    fn update_layout(&self) -> windows::core::Result<()> {
        let target_rect = self.target.rect()?;
        unsafe {
            MoveWindow(
                self.hwnd,
                0,
                0,
                target_rect.width(),
                target_rect.height(),
                true,
            )
        }
        .ok()?;

        Ok(())
    }
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let map = OVERLAY_INSTANCES.read().unwrap_or_log();
    let overlay = if let Some(overlay) = map.get(&hwnd.0) {
        overlay
    } else {
        return DefWindowProcW(hwnd, msg, wparam, lparam);
    };

    // TODO: refactor this
    match msg {
        WM_DPICHANGED => {
            overlay.update_layout().unwrap_or_log();
            LRESULT(0)
        }
        WM_PAINT => {
            let mut rect = RECT::default();
            GetClientRect(hwnd, &mut rect);
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);
            SetBkMode(&hdc, TRANSPARENT);

            let taskbar_rect = overlay.target.rect().unwrap();
            let width = taskbar_rect.width();
            let height = taskbar_rect.height();
            let data: Vec<u32> = if let Some(pixmap) = &overlay.pixmap {
                pixmap
                    .pixels()
                    .iter()
                    .map(|color| {
                        ((color.blue() as u32) << 0)
                            | ((color.green() as u32) << 8)
                            | ((color.red() as u32) << 16)
                            | ((color.alpha() as u32) << 24)
                    })
                    .collect()
            } else {
                let color = overlay.background_color;
                vec![color; (width * height) as usize]
            };
            let dc = CreateCompatibleDC(hdc);
            let bitmap = CreateBitmap(width, height, 1, 32, data.as_ptr() as _);
            SelectObject(dc, bitmap);
            FillRect(&hdc, &rect, CreateSolidBrush(overlay.background_color));
            BitBlt(hdc, 0, 0, width, height, dc, 0, 0, SRCPAINT);
            DeleteObject(bitmap);
            DeleteDC(dc);

            EndPaint(hwnd, &ps);
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
