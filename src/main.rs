#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use std::time::Duration;
use std::{fs, thread};

use config::MeasurrredConfig;
use data_source::{BoxedDataSource, GlobalMemoryStatusDataSource, PdhDataSource};

use platform::taskbar::{TaskbarHandle, TaskbarOverlay};
use tiny_skia::{Paint, Pixmap, Rect, Transform};
use tracing_unwrap::ResultExt;
use usvg::Options;
use widget::load_widget;

mod component;
mod config;
mod data_source;
mod platform;
mod system;
mod widget;

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let config = MeasurrredConfig {
        foreground_color: "white".parse().unwrap_or_log(),
        background_color: "black".parse().unwrap_or_log(),
        font_family: "Noto Sans CJK KR Bold".to_string(),
    };

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

    let taskbar = TaskbarHandle::collect()?.remove(0);
    let mut overlay = TaskbarOverlay::new(taskbar)?;
    overlay.accept_config(&config)?;
    overlay.show();

    let mut options = Options::default();
    options.fontdb.load_system_fonts();

    let local_appdata = std::env::var("LocalAppdata").unwrap();
    options
        .fontdb
        .load_fonts_dir(std::path::PathBuf::from(local_appdata).join("Microsoft/Windows/Fonts"));
    let mut overlay_w = overlay.clone();
    let handle = thread::spawn(move || loop {
        let taskbar_rect = overlay_w.target.rect().unwrap();
        let width = taskbar_rect.width();
        let height = taskbar_rect.height();
        let mut pixmap = Pixmap::new(width as u32, height as u32).unwrap();
        let mut paint = Paint::default();
        paint.set_color(config.background_color.to_tiny_skia_color());
        pixmap.fill_rect(
            Rect::from_xywh(0.0, 0.0, width as f32, height as f32).unwrap(),
            &paint,
            Transform::default(),
            None,
        );
        for widget in widgets.iter_mut() {
            widget
                .render(&config, &options, &mut pixmap)
                .unwrap_or_log();
        }
        overlay_w.accept_pixmap(pixmap).unwrap_or_log();
        overlay_w.redraw().unwrap_or_log();
        for data_source in context.data_source.values_mut() {
            data_source.update().unwrap_or_log();
        }
        thread::sleep(Duration::from_millis(1000));
    });

    overlay.begin_event_loop()?;

    handle.join().expect("Should join the thread updating");

    Ok(())
}
