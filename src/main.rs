#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ptr::null_mut;
use std::thread;
use std::time::Duration;

use taskbar_overlay::TaskbarOverlay;

use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};

mod component;
mod data_source;
mod position;
mod taskbar;
mod taskbar_overlay;

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        ((($r) | (($g) << 8)) | (($b) << 16))
    };
}

fn main() -> eyre::Result<()> {
    unsafe {
        CoInitializeEx(null_mut(), COINIT_APARTMENTTHREADED)?;
    }

    let overlay = TaskbarOverlay::try_initialize()?;
    overlay.show();

    let handle = thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(2000));
        overlay.update().expect("Should update successfully");
    });

    overlay.begin_event_loop()?;

    handle.join().expect("Should join the thread updating");

    Ok(())
}
