use std::ptr::null_mut;

use once_cell::sync::OnceCell;
use windows::Win32::{
    Foundation::{GetLastError, HWND, LPARAM, LRESULT, PWSTR, RECT, WPARAM},
    Graphics::Gdi::{
        BeginPaint, CreateSolidBrush, EndPaint, FillRect, RedrawWindow, SetBkMode, SetTextColor,
        TextOutW, UpdateWindow, PAINTSTRUCT, RDW_INVALIDATE, RDW_UPDATENOW, TRANSPARENT,
    },
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, GetClientRect, PostQuitMessage, RegisterClassW,
        SetLayeredWindowAttributes, ShowWindow, CS_HREDRAW, CS_VREDRAW, LWA_COLORKEY, SW_SHOW,
        WM_DESTROY, WM_PAINT, WNDCLASSW, WS_CHILD, WS_EX_LAYERED, WS_EX_TOPMOST, WS_VISIBLE,
    },
};

use crate::taskbar::Taskbar;

static TASKBAR_OVERLAY: OnceCell<TaskbarOverlay> = OnceCell::new();

pub struct TaskbarOverlay {
    target: Taskbar,
    window: HWND,
    background_color: u32,
    transparent_background: bool,
}

impl TaskbarOverlay {
    pub fn try_initialize() -> eyre::Result<&'static TaskbarOverlay> {
        TASKBAR_OVERLAY.get_or_try_init(|| {
            let taskbar = Taskbar::try_initialize()?;
            let taskbar_rect = taskbar.rect()?;

            let instance = unsafe { GetModuleHandleW(PWSTR(null_mut())) };
            if instance == 0 {
                eyre::bail!(
                    "Failed to get the handle of this module: Error Code 0x{:x}",
                    unsafe { GetLastError() }
                );
            }

            let class_name = "MeasurredTaskbar";
            let class_name = PWSTR(class_name.as_ptr() as _);
            let class = WNDCLASSW {
                lpfnWndProc: Some(wndproc),
                hInstance: instance,
                lpszClassName: class_name,
                style: CS_HREDRAW | CS_VREDRAW,
                ..Default::default()
            };

            let class_id = unsafe { RegisterClassW(&class) };
            if class_id == 0 {
                eyre::bail!(
                    "Failed to register window class: Error Code 0x{:x}",
                    unsafe { GetLastError() }
                );
            }

            let window_handle = unsafe {
                CreateWindowExW(
                    WS_EX_TOPMOST | WS_EX_LAYERED,
                    class_name,
                    "measurrred",
                    WS_VISIBLE | WS_CHILD,
                    0,
                    0,
                    taskbar_rect.right - taskbar_rect.left,
                    taskbar_rect.bottom - taskbar_rect.top,
                    taskbar.0,
                    0,
                    instance,
                    null_mut(),
                )
            };
            if window_handle == 0 {
                eyre::bail!("Failed to create the window: Error Code 0x{:x}", unsafe {
                    GetLastError()
                });
            }

            let overlay = TaskbarOverlay {
                target: taskbar,
                window: window_handle,
                background_color: 0x000000,
                transparent_background: true,
            };

            overlay.update_background()?;

            Ok(overlay)
        })
    }

    pub fn set_background_color(&mut self, red: u32, green: u32, blue: u32) -> eyre::Result<()> {
        let color = red | (green << 8) | (blue << 16);
        self.background_color = color;
        self.update_background()
    }

    pub fn update_background(&self) -> eyre::Result<()> {
        if self.transparent_background {
            let fail = unsafe {
                SetLayeredWindowAttributes(self.window, self.background_color, 0, LWA_COLORKEY).0
                    == 0
            };
            if fail {
                eyre::bail!(
                    "Failed to make window background transparent: Error Code 0x{:x}",
                    unsafe { GetLastError() }
                );
            }
        }
        Ok(())
    }

    pub fn show(&self) -> bool {
        unsafe { ShowWindow(self.window, SW_SHOW).as_bool() }
    }

    pub fn update(&self) -> eyre::Result<()> {
        let fail = unsafe {
            RedrawWindow(self.window, null_mut(), 0, RDW_INVALIDATE | RDW_UPDATENOW).0 == 0
        };
        if fail {
            eyre::bail!("Failed to update the window: Error Code 0x{:x}", unsafe {
                GetLastError()
            });
        }

        Ok(())
    }
}

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        ((($r) | (($g) << 8)) | (($b) << 16))
    };
}

unsafe extern "system" fn wndproc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    static mut test: u64 = 0;

    let overlay = match TASKBAR_OVERLAY.get() {
        Some(overlay) => overlay,
        None => {
            return DefWindowProcW(window, message, wparam, lparam);
        }
    };

    match message as u32 {
        WM_PAINT => {
            let mut rect = RECT::default();
            GetClientRect(window, &mut rect);
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(window, &mut ps);
            FillRect(hdc, &rect, CreateSolidBrush(overlay.background_color));
            SetBkMode(hdc, TRANSPARENT);
            SetTextColor(hdc, rgb!(255, 0, 0));
            test += 1;
            let text = format!("{}", test);
            let text: &str = &text;
            TextOutW(hdc, 16, 16, text, text.len() as _);
            EndPaint(hdc, &ps);
            0
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(window, message, wparam, lparam),
    }
}
