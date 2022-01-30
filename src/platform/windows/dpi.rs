use once_cell::sync::OnceCell;
use windows::Win32::UI::HiDpi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE};

pub fn become_dpi_aware() -> windows::core::Result<()> {
    static DPI_AWARE: OnceCell<()> = OnceCell::new();

    if DPI_AWARE.get().is_some() {
        Ok(())
    } else {
        DPI_AWARE
            .get_or_try_init(|| unsafe { SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE) })
            .map(|_| ())
    }
}
