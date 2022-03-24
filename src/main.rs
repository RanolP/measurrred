#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use std::ptr::null_mut;
use std::time::{Duration, Instant};
use std::{fs, thread};

use measurrred::system::HorizontalPosition;
use tiny_skia::{Paint, Pixmap, Rect, Transform};
use tracing::{error, info, warn};
use tracing_unwrap::ResultExt;
use usvg::Options;

use measurrred::platform::taskbar::{TaskbarHandle, TaskbarOverlay};
use measurrred::widget::load_widget;
use measurrred::{
    component::SetupContext,
    config::MeasurrredConfig,
    data_source::{
        BatteryReportDataSource, BoxedDataSource, GlobalMemoryStatusDataSource, PdhDataSource,
    },
};
use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};

mod log;

fn main() -> eyre::Result<()> {
    log::initialize_tracing_logger();
    // unsafe { CoInitializeEx(null_mut(), COINIT_MULTITHREADED) }?;

    info!("Starting");

    let measurrred_config = MeasurrredConfig::load()?;

    info!("Config loaded.");

    let data_source_list: Vec<BoxedDataSource> = vec![
        Box::new(PdhDataSource::new().unwrap_or_log()),
        Box::new(GlobalMemoryStatusDataSource),
        Box::new(BatteryReportDataSource),
    ];
    let data_source = HashMap::from_iter(
        data_source_list
            .into_iter()
            .map(|data_source| (data_source.name(), data_source)),
    );

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
            Ok(Some(widget)) => widget,
            Ok(None) => {
                warn!("{} is disabled.", directory.to_string_lossy());
                continue;
            }
            Err(e) => {
                error!(
                    "Skipping directory {} due to an error: {}",
                    directory.to_string_lossy(),
                    e
                );
                continue;
            }
        };

        widgets.push(widget);
    }

    let mut usvg_options = Options::default();
    usvg_options.fontdb.load_system_fonts();

    let mut context = SetupContext {
        data_source,
        usvg_options,
    };

    for widget in widgets.iter_mut() {
        widget.setup(&mut context)?;
    }

    let SetupContext {
        mut data_source,
        usvg_options,
    } = context;

    let taskbar = TaskbarHandle::collect()?.remove(0);

    let mut overlay = TaskbarOverlay::new(taskbar)?;
    overlay.accept_config(&measurrred_config)?;
    overlay.show();

    info!("Hello, measurrred!");

    let mut overlay_w = overlay.clone();
    let handle = thread::spawn(move || -> eyre::Result<()> {
        loop {
            let begin = Instant::now();

            let taskbar_rect = overlay_w.target.rect()?;
            let width = taskbar_rect.width();
            let height = taskbar_rect.height();
            let mut pixmap = Pixmap::new(width as u32, height as u32).unwrap();
            let mut paint = Paint::default();
            paint.set_color(
                measurrred_config
                    .general
                    .background_color
                    .to_tiny_skia_color(),
            );
            pixmap.fill_rect(
                Rect::from_xywh(0.0, 0.0, width as f32, height as f32).unwrap(),
                &paint,
                Transform::default(),
                None,
            );
            let zoom = overlay_w.zoom()?;
            for widget in widgets.iter_mut() {
                widget.render(
                    &measurrred_config,
                    &usvg_options,
                    &mut pixmap,
                    match widget.x {
                        HorizontalPosition::Right(_)
                            if measurrred_config
                                .viewbox_tuning
                                .respect_tray_area_when_right_align =>
                        {
                            overlay_w.target.rebar_rect()?
                        }
                        _ => overlay_w.target.rect()?,
                    },
                    zoom,
                )?;
            }
            overlay_w.accept_pixmap(pixmap)?;
            overlay_w.redraw()?;
            for data_source in data_source.values_mut() {
                data_source.update()?;
            }

            let delta = begin.elapsed().as_millis() as u64;

            if delta >= measurrred_config.general.refresh_interval {
                warn!(
                "Rendered in {} ms (>= refresh-interval). Consider higher refresh-interval value.",
                delta
            )
            } else {
                thread::sleep(Duration::from_millis(
                    measurrred_config.general.refresh_interval - delta,
                ));
            }
        }
    });

    overlay.begin_event_loop()?;

    if let Err(e) = handle.join().unwrap() {
        info!(
            "Got an error while joining updator handle but it is normal: {}",
            e
        );
    }

    overlay.shutdown()?;

    Ok(())
}
