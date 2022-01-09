// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ptr::null_mut;
use std::thread;
use std::time::Duration;

use taskbar_overlay::TaskbarOverlay;

use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};

use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, TranslateMessage, MSG,
};

mod taskbar;
mod taskbar_overlay;

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

    let overlay = TaskbarOverlay::try_initialize()?;
    overlay.show();

    let handle = thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(1000));
        overlay.update().expect("Should update successfully");
    });

    let mut message = MSG::default();
    while GetMessageW(&mut message, 0, 0, 0).0 > 0 {
        TranslateMessage(&message);
        DispatchMessageW(&mut message);
    }

    handle.join();

    Ok(())
}
