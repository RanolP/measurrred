#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ptr::null_mut;
use std::thread;
use std::time::{Duration, Instant};

use windows::Win32::Foundation::{BOOL, HWND, LPARAM, LRESULT, PWSTR, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    BeginPaint, BitBlt, CreateSolidBrush, EndPaint, FillRect, GetDC, RedrawWindow, ReleaseDC,
    SetBkMode, SetTextColor, TextOutW, UpdateWindow, ValidateRect, PAINTSTRUCT, RDW_INVALIDATE,
    RDW_UPDATENOW, TRANSPARENT,
};
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, FindWindowExW, FindWindowW, GetClientRect,
    GetMessageW, GetWindowRect, PostQuitMessage, RegisterClassW, SendMessageW,
    SetLayeredWindowAttributes, ShowWindow, TranslateMessage, CS_HREDRAW, CS_VREDRAW, LWA_COLORKEY,
    MSG, SW_SHOW, WM_CREATE, WM_DESTROY, WM_ERASEBKGND, WM_PAINT, WNDCLASSW, WS_BORDER, WS_CHILD,
    WS_EX_LAYERED, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_OVERLAPPEDWINDOW, WS_POPUP, WS_VISIBLE,
};

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        ((($r) | (($g) << 8)) | (($b) << 16))
    };
}

fn main() -> eyre::Result<()> {
    unsafe { real_main() }
}

unsafe fn real_main() -> eyre::Result<()> {
    CoInitializeEx(null_mut(), COINIT_APARTMENTTHREADED)?;

    let instance = GetModuleHandleW(PWSTR(null_mut()));
    let taskbar_handle = FindWindowW("Shell_TrayWnd", PWSTR(null_mut()));
    let bar_handle = FindWindowExW(taskbar_handle, 0, "ReBarWindow32", PWSTR(null_mut()));

    #[allow(unused_variables)]
    let minimize_handle = FindWindowExW(bar_handle, 0, "MSTaskSwWClass", PWSTR(null_mut()));

    let name = "MeasurredTaskbar";
    let name = PWSTR(name.as_ptr() as _);
    let class = WNDCLASSW {
        lpfnWndProc: Some(wndproc),
        hInstance: instance,
        lpszClassName: name,
        style: CS_HREDRAW | CS_VREDRAW,
        hbrBackground: CreateSolidBrush(rgb!(0, 0, 0)),
        ..Default::default()
    };
    RegisterClassW(&class);

    let window = CreateWindowExW(
        WS_EX_TOPMOST | WS_EX_LAYERED,
        name,
        "measurrred",
        WS_VISIBLE | WS_CHILD,
        0,
        0,
        800,
        600,
        bar_handle,
        0,
        instance,
        null_mut(),
    );

    SetLayeredWindowAttributes(window, rgb!(0, 0, 0), 0, LWA_COLORKEY);

    ShowWindow(window, SW_SHOW);
    UpdateWindow(window);

    let mut message = MSG::default();

    let handle = thread::spawn(move || {
        while true {
            RedrawWindow(window, null_mut(), 0, RDW_INVALIDATE | RDW_UPDATENOW);
            thread::sleep(Duration::from_millis(1000));
        }
    });

    while GetMessageW(&mut message, 0, 0, 0).0 > 0 {
        TranslateMessage(&message);
        DispatchMessageW(&mut message);
    }

    handle.join();

    Ok(())
}

static mut test: u64 = 0;

unsafe extern "system" fn wndproc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message as u32 {
        WM_PAINT => {
            let mut rect = RECT::default();
            GetClientRect(window, &mut rect);
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(window, &mut ps);
            FillRect(hdc, &rect, CreateSolidBrush(rgb!(0, 0, 0)));
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
