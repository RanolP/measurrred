#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Read;
use std::time::Duration;
use std::{collections::HashMap, ptr::null_mut};
use std::{fs, thread};

use component::Component;
use data_source::{BoxedDataSource, GlobalMemoryStatusDataSource, PdhDataSource};

use taskbar_overlay::TaskbarOverlay;

use widget::{Widget, WidgetConfig};
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};
use windows::Win32::System::Performance::PdhEnumObjectItemsW;

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

    let mut widgets = Vec::new();

    for directory in fs::read_dir("widgets")
        .and_then(|it| it.collect::<Result<Vec<_>, _>>())
        .map(|it| {
            it.iter()
                .flat_map(|dir| {
                    fs::read_dir(dir.path())
                        .and_then(|it| it.collect::<Result<Vec<_>, _>>())
                        .unwrap_or(Vec::new())
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or(Vec::new())
    {
        let directory = directory.path();
        let mut taskbar_config = match fs::File::open(directory.join("taskbar.config.toml")) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Skipping directory {}", directory.to_string_lossy());
                continue;
            }
        };
        let mut taskbar_component = match fs::File::open(directory.join("taskbar.component.xml")) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Skipping directory {}", directory.to_string_lossy());
                continue;
            }
        };

        let taskbar_config = match {
            let mut buf = String::new();
            taskbar_config
                .read_to_string(&mut buf)
                .map_err(eyre::Report::from)
                .and_then(|_| toml::from_str::<WidgetConfig>(&buf).map_err(eyre::Report::from))
        } {
            Ok(conf) => conf,
            Err(e) => {
                eprintln!(
                    "Failed to read file {}/taskbar.config.toml; {}",
                    directory.to_string_lossy(),
                    e
                );
                continue;
            }
        };

        let taskbar_component = match {
            let mut buf = String::new();
            taskbar_component
                .read_to_string(&mut buf)
                .map_err(eyre::Report::from)
                .and_then(|_| {
                    quick_xml::de::from_str::<Component>(&buf).map_err(eyre::Report::from)
                })
        } {
            Ok(conf) => conf,
            Err(e) => {
                eprintln!(
                    "Failed to read file {}/taskbar.component.xml; {}",
                    directory.to_string_lossy(),
                    e
                );
                continue;
            }
        };

        let widget = Widget::new(taskbar_config, vec![taskbar_component]);
        widgets.push(widget);
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

    for widget in widgets.iter_mut() {
        widget.setup(&mut context)?;
    }

    let overlay = TaskbarOverlay::try_initialize(widgets)?;
    overlay.show();

    let handle = thread::spawn(move || loop {
        overlay.update().expect("Should update successfully");
        for data_source in context.data_source.values_mut() {
            data_source.update().expect("Should update successfully");
        }
        thread::sleep(Duration::from_millis(1000));
    });

    overlay.begin_event_loop()?;

    handle.join().expect("Should join the thread updating");

    Ok(())
}
