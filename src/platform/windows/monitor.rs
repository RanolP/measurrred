use windows::Win32::{
    Foundation::HWND,
    Graphics::Gdi::{MonitorFromWindow, HMONITOR, MONITOR_DEFAULTTONEAREST},
    UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI},
};

pub struct MonitorHandle {
    hmonitor: HMONITOR,
}

impl MonitorHandle {
    pub fn from_hwnd(hwnd: HWND) -> MonitorHandle {
        let hmonitor = unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST) };
        MonitorHandle { hmonitor }
    }
    pub fn get_dpi(&self) -> windows::core::Result<u32> {
        let mut dpi_x = 0;
        let mut dpi_y = 0;
        unsafe { GetDpiForMonitor(self.hmonitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y) }?;
        Ok(dpi_x)
    }
}
