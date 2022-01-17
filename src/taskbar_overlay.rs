use std::ptr::null_mut;

use once_cell::sync::{Lazy, OnceCell};
use tiny_skia::{Paint, Pixmap, PremultipliedColorU8, Rect, Transform};
use windows::Win32::{
    Foundation::{GetLastError, HWND, LPARAM, LRESULT, PWSTR, RECT, WPARAM},
    Graphics::Gdi::{
        BeginPaint, BitBlt, CreateBitmap, CreateCompatibleDC, CreateSolidBrush, DeleteDC,
        DeleteObject, EndPaint, FillRect, RedrawWindow, SelectObject, SetBkMode, SetTextColor,
        TextOutW, HGDIOBJ, HRGN, MERGECOPY, MERGEPAINT, PAINTSTRUCT, RDW_INVALIDATE, RDW_UPDATENOW,
        SRCPAINT, TRANSPARENT,
    },
    System::{
        LibraryLoader::GetModuleHandleW,
        Performance::{
            PdhAddEnglishCounterW, PdhCollectQueryData, PdhGetFormattedCounterValue, PdhOpenQueryW,
            PDH_FMT_COUNTERVALUE, PDH_FMT_DOUBLE,
        },
        SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX},
    },
    UI::WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, DispatchMessageW, GetClientRect, GetMessageW,
        PostQuitMessage, RegisterClassW, SetLayeredWindowAttributes, ShowWindow, TranslateMessage,
        CS_HREDRAW, CS_VREDRAW, HMENU, LWA_COLORKEY, MSG, SW_SHOW, WM_DESTROY, WM_ERASEBKGND,
        WM_PAINT, WNDCLASSW, WS_CHILD, WS_EX_LAYERED, WS_EX_TOPMOST, WS_VISIBLE,
    },
};

use crate::{
    component::Component,
    data_source::{Data, DataSource, PdhDataSource, PreferredDataType},
    system::{HorizontalPosition, Length, VerticalPosition},
    taskbar::Taskbar,
    widget::Widget,
};

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

            let instance = unsafe { GetModuleHandleW(PWSTR(null_mut())) }.ok()?;

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
                Err(::windows::core::Error::from_win32())?;
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
                    HMENU(0),
                    instance,
                    null_mut(),
                )
            }
            .ok()?;

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
            unsafe {
                SetLayeredWindowAttributes(self.window, self.background_color, 0, LWA_COLORKEY)
            }
            .ok()?;
        }
        Ok(())
    }

    pub fn show(&self) -> bool {
        unsafe { ShowWindow(self.window, SW_SHOW).as_bool() }
    }

    pub fn update(&self) -> eyre::Result<()> {
        let fail = unsafe {
            RedrawWindow(
                self.window,
                null_mut(),
                HRGN(0),
                RDW_INVALIDATE | RDW_UPDATENOW,
            )
            .0 == 0
        };
        if fail {
            eyre::bail!("Failed to update the window: Error Code 0x{:x}", unsafe {
                GetLastError()
            });
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
                eyre::bail!(
                    "Failed to get message on overlay window: Error Code 0x{:x}",
                    unsafe { GetLastError() }
                );
            }
            unsafe {
                TranslateMessage(&message);
                // its return value, LRESULT, is generally ignored
                DispatchMessageW(&mut message);
            }
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
    static mut WIDGET: Lazy<Widget> = Lazy::new(|| Widget {
        x: HorizontalPosition::Left(Length::Pixel(8)),
        y: VerticalPosition::Center,
        components: vec![Component::HBox()],
    });

    let overlay = match TASKBAR_OVERLAY.get() {
        Some(overlay) => overlay,
        None => {
            return DefWindowProcW(window, message, wparam, lparam);
        }
    };

    let mut mem = MEMORYSTATUSEX::default();
    mem.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;

    if GlobalMemoryStatusEx(&mut mem).0 == 0 {
        println!("wtf, {}", GetLastError());
    }

    match message as u32 {
        WM_PAINT => {
            let mut rect = RECT::default();
            GetClientRect(window, &mut rect);
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(window, &mut ps);
            SetBkMode(&hdc, TRANSPARENT);

            let taskbar_rect = Taskbar::get().unwrap().rect().unwrap();
            let width = taskbar_rect.right - taskbar_rect.left;
            let height = taskbar_rect.bottom - taskbar_rect.top;
            let mut pixmap = Pixmap::new(width as u32, height as u32).unwrap();
            let paint = Paint::default();
            pixmap.fill_rect(
                Rect::from_xywh(0.0, 0.0, width as f32, height as f32).unwrap(),
                &paint,
                Transform::default(),
                None,
            );
            WIDGET.render(&mut pixmap).unwrap();
            let data: Vec<u32> = pixmap
                .pixels()
                .iter()
                .map(|color| {
                    ((color.blue() as u32) << 0)
                        | ((color.green() as u32) << 8)
                        | ((color.red() as u32) << 16)
                        | ((color.alpha() as u32) << 24)
                })
                .collect();
            let dc = CreateCompatibleDC(hdc);
            let bitmap = CreateBitmap(width, height, 1, 32, data.as_ptr() as _);
            SelectObject(dc, bitmap);
            FillRect(&hdc, &rect, CreateSolidBrush(overlay.background_color));
            BitBlt(hdc, 0, 0, width, height, dc, 0, 0, SRCPAINT);
            DeleteObject(bitmap);
            DeleteDC(dc);

            EndPaint(window, &ps);
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(window, message, wparam, lparam),
    }
}
