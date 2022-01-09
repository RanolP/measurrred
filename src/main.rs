// #![windows_subsystem = "windows"]

use std::ptr::null_mut;

use windows::Win32::Foundation::{GetLastError, BOOL, HWND, LPARAM, LRESULT, PWSTR, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{BeginPaint, EndPaint, TextOutW, UpdateWindow, PAINTSTRUCT};
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::WinRT::Xaml::{
    IDesktopWindowXamlSourceNative, IDesktopWindowXamlSourceNative2,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, EnumChildWindows, FindWindowExW,
    FindWindowW, GetClassNameW, GetClientRect, GetMessageW, GetWindowLongPtrW, MoveWindow,
    PostQuitMessage, RegisterClassW, SendMessageW, SetLayeredWindowAttributes, SetWindowLongPtrW,
    SetWindowPos, ShowWindow, TranslateMessage, CS_HREDRAW, CS_VREDRAW, GWL_EXSTYLE, LWA_COLORKEY,
    MSG, SWP_SHOWWINDOW, SW_SHOW, WM_CREATE, WM_DESTROY, WM_ERASEBKGND, WM_PAINT, WM_SIZE,
    WNDCLASSW, WS_BORDER, WS_CHILD, WS_EX_COMPOSITED, WS_EX_LAYERED, WS_EX_TOOLWINDOW,
    WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
};
use windows::UI::Xaml::Hosting::DesktopWindowXamlSource;
use windows::UI::Xaml::Media::{AcrylicBackgroundSource, AcrylicBrush, Brush, SolidColorBrush};
use windows::UI::Xaml::{FrameworkElement, UIElement};
use windows::UI::{Color, Colors};
use windows::{
    core::*,
    UI::Xaml::Controls::{Panel, StackPanel, TextBox},
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
        ..Default::default()
    };
    RegisterClassW(&class);

    let taskbar_mode = true;

    let window = CreateWindowExW(
        WS_EX_TOPMOST | WS_EX_LAYERED,
        name,
        "measurrred",
        if taskbar_mode {
            WS_VISIBLE | WS_CHILD
        } else {
            WS_VISIBLE | WS_OVERLAPPEDWINDOW
        },
        0,
        0,
        800,
        600,
        if taskbar_mode { bar_handle } else { 0 },
        0,
        instance,
        null_mut(),
    );

    let source: DesktopWindowXamlSource = DesktopWindowXamlSource::new()?;

    let textbox = TextBox::new()?;
    textbox.SetText("Hello, world!")?;

    let container = StackPanel::new()?;
    let container: Panel = container.cast()?;
    container.SetBackground({
        let brush = AcrylicBrush::new()?;

        brush.SetBackgroundSource(AcrylicBackgroundSource::HostBackdrop)?;

        brush
    })?;

    container.Children()?.Append(&textbox)?;

    let container: UIElement = container.cast()?;
    container.UpdateLayout()?;

    source.SetContent(&container)?;

    let native: IDesktopWindowXamlSourceNative2 = source.cast()?;
    native.AttachToWindow(window)?;
    let native_handle = native.WindowHandle()?;
    SetWindowPos(native_handle, 0, 0, 0, 600, 400, SWP_SHOWWINDOW);
    ShowWindow(native_handle, SW_SHOW);

    SendMessageW(native_handle, WM_ERASEBKGND, 1, 0);

    ShowWindow(window, SW_SHOW);
    UpdateWindow(window);

    let mut buf = [0u16; 1000];
    GetClassNameW(native_handle, PWSTR(buf.as_mut_ptr()), 1000);
    println!(
        "{} -> {}",
        native_handle,
        String::from_utf16(&buf).unwrap_or("FAILED TO FETCH".into())
    );
    let old_ex_style = GetWindowLongPtrW(native_handle, GWL_EXSTYLE);
    SetWindowLongPtrW(
        native_handle,
        GWL_EXSTYLE,
        old_ex_style | WS_EX_LAYERED as isize | WS_EX_TOOLWINDOW as isize,
    );
    SetLayeredWindowAttributes(native_handle, rgb!(255, 0, 0), 0, LWA_COLORKEY);

    EnumChildWindows(native_handle, Some(enumerate), 0);

    unsafe extern "system" fn enumerate(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let mut buf = [0u16; 1000];
        GetClassNameW(hwnd, PWSTR(buf.as_mut_ptr()), 1000);
        println!(
            "{} -> {}",
            hwnd,
            String::from_utf16(&buf).unwrap_or("FAILED TO FETCH".into())
        );
        true.into()
    }

    let mut message = MSG::default();

    while GetMessageW(&mut message, 0, 0, 0).0 > 0 {
        let mut xaml_source_processed_message = BOOL::from(false);
        native.PreTranslateMessage(&message, &mut xaml_source_processed_message)?;
        if !xaml_source_processed_message.as_bool() {
            TranslateMessage(&message);
            DispatchMessageW(&mut message);
        }
    }

    Ok(())
}

unsafe extern "system" fn wndproc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message as u32 {
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(window, message, wparam, lparam),
    }
}
