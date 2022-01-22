#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use std::time::Duration;
use std::{fs, thread};

use data_source::{BoxedDataSource, GlobalMemoryStatusDataSource, PdhDataSource};

use taskbar_overlay::TaskbarOverlay;

use widget::load_widget;

mod component;
mod config;
mod data_source;
mod platform;
mod system;
mod taskbar_overlay;
mod widget;

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();
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
        let widget = match load_widget(&directory) {
            Ok(widget) => widget,
            Err(e) => {
                eprintln!(
                    "Skipping directory {} due to an error\n└ {}",
                    directory.to_string_lossy(),
                    e
                );
                continue;
            }
        };

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
