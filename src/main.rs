#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;
use std::time::Duration;
use std::{collections::HashMap, ptr::null_mut};

use component::Component;
use data_source::{BoxedDataSource, GlobalMemoryStatusDataSource, PdhDataSource};
use system::{HorizontalPosition, Length, VerticalPosition};
use taskbar_overlay::TaskbarOverlay;

use widget::Widget;
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};

mod component;
mod config;
mod data_source;
mod system;
mod taskbar;
mod taskbar_overlay;
mod widget;

fn main() -> eyre::Result<()> {
    unsafe {
        CoInitializeEx(null_mut(), COINIT_APARTMENTTHREADED)?;
    }

    let data_source_list: Vec<BoxedDataSource> = vec![
        Box::new(PdhDataSource::new().unwrap()),
        Box::new(GlobalMemoryStatusDataSource),
    ];
    let data_source = HashMap::from_iter(
        data_source_list
            .into_iter()
            .map(|data_source| (data_source.name(), data_source)),
    );

    let mut context = component::SetupContext { data_source };

    let src = r#"
        <vbox>
            <hbox y-align="center">
                <text color="yellow">CPU</text>
                <margin size="8" />
                <data-text source="pdh" query="\Processor(_Total)\% Processor Time" format="float" />
                <margin size="2" />
                <text>%</text>
            </hbox>
            <hbox y-align="center">
                <text color="yellow">RAM</text>
                <margin size="8" />
                <data-text source="global-memory-status" query="dMemoryLoad" format="float" />
                <margin size="2" />
                <text>%</text>
                <margin size="6" />
                <text>(</text>
                <data-text source="global-memory-status" query="ullUsedPhys" format="float" divide-by="1073741824" />
                <margin size="4" />
                <text>/</text>
                <margin size="4" />
                <data-text source="global-memory-status" query="ullTotalPhys" format="float" divide-by="1073741824" />
                <margin size="4" />
                <text>GB</text>
                <text>)</text>
            </hbox>
        </vbox>
    "#;

    let mut widgets = vec![Widget {
        x: HorizontalPosition::Left(Length::Pixel(16)),
        y: VerticalPosition::Center,
        components: vec![quick_xml::de::from_str::<Component>(src).unwrap()],
    }];

    for widget in widgets.iter_mut() {
        widget.setup(&mut context)?;
    }

    let overlay = TaskbarOverlay::try_initialize(widgets)?;
    overlay.show();

    let handle = thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(2000));
        overlay.update().expect("Should update successfully");
        for data_source in context.data_source.values_mut() {
            data_source.update().expect("Should update successfully");
        }
    });

    overlay.begin_event_loop()?;

    handle.join().expect("Should join the thread updating");

    Ok(())
}
