#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use std::time::Duration;
use std::{fs, thread};

use config::MeasurrredConfig;
use data_source::{BoxedDataSource, GlobalMemoryStatusDataSource, PdhDataSource};

use platform::taskbar::{TaskbarHandle, TaskbarOverlay};
use tiny_skia::{Paint, Pixmap, Rect, Transform};
use tracing::{error, info};
use tracing_unwrap::ResultExt;
use usvg::Options;
use widget::load_widget;

mod component;
mod config;
mod data_source;
mod platform;
mod system;
mod util;
mod widget;

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    info!("Starting");

    let config = MeasurrredConfig::load()?;

    info!("Config loaded.");

    info!("Initializing widgets");
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

        info!("Visiting directory {}", directory.to_string_lossy());

        let widget = match load_widget(&directory) {
            Ok(widget) => widget,
            Err(e) => {
                error!(
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
        Box::new(PdhDataSource::new().unwrap_or_log()),
        Box::new(GlobalMemoryStatusDataSource),
    ];
    let data_source = HashMap::from_iter(
        data_source_list
            .into_iter()
            .map(|data_source| (data_source.name(), data_source)),
    );

    let mut usvg_options = Options::default();
    usvg_options.fontdb.load_system_fonts();

    if cfg!(target_os = "windows") {
        let local_appdata = std::env::var("LocalAppdata").unwrap();
        usvg_options.fontdb.load_fonts_dir(
            std::path::PathBuf::from(local_appdata).join("Microsoft/Windows/Fonts"),
        );
    }
    let mut context = component::SetupContext {
        data_source,
        usvg_options,
    };

    for widget in widgets.iter_mut() {
        widget.setup(&mut context)?;
    }

    let options = context.usvg_options;

    let taskbar = TaskbarHandle::collect()?.remove(0);
    let mut overlay = TaskbarOverlay::new(taskbar)?;
    overlay.accept_config(&config)?;
    overlay.show();

    info!("Hello, measurrred!");

    let mut overlay_w = overlay.clone();
    let zoom = overlay_w.zoom()?;
    let handle = thread::spawn(move || loop {
        let taskbar_rect = overlay_w.target.rect().unwrap_or_log();
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
                .render(&config, &options, &mut pixmap, zoom)
                .unwrap_or_log();
        }
        overlay_w.accept_pixmap(pixmap).unwrap_or_log();
        overlay_w.redraw().unwrap_or_log();
        for data_source in context.data_source.values_mut() {
            data_source.update().unwrap_or_log();
        }
        thread::sleep(Duration::from_millis(config.refresh_interval));
    });

    overlay.begin_event_loop()?;

    handle.join().expect("Should join the thread updating");

    Ok(())
}
