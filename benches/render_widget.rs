use criterion::{black_box, criterion_group, criterion_main, Criterion};
use measurrred::{
    component::ComponentAction,
    system::{HorizontalPosition, Length, Rect, VerticalPosition},
    widget::Widget,
};
use tiny_skia::Pixmap;

mod mock {
    use std::str::FromStr;

    use measurrred::{
        component::{Component, EitherVariable, RenderContext, SetupContext, Text, UpdateContext},
        config::{GeneralSection, MeasurrredConfig, ViewboxTuningSection},
        system::Color,
    };
    use usvg::Options;
    pub fn text() -> Component {
        Component::Text(Text {
            color: None,
            text_align: Default::default(),
            font_size: None,
            font_family: None,
            font_weight: None,
            content: vec![EitherVariable::T("Test".to_string())],
        })
    }

    pub fn config() -> MeasurrredConfig {
        MeasurrredConfig {
            general: GeneralSection {
                foreground_color: Color::from_str("white").unwrap(),
                background_color: Color::from_str("black").unwrap(),
                font_family: "Arial".to_string(),
                font_weight: None,
                refresh_interval: 1000,
            },
            viewbox_tuning: ViewboxTuningSection {
                respect_tray_area_when_right_align: true,
            },
        }
    }

    pub fn usvg_options() -> Options {
        let mut options = Options::default();
        options.fontdb.load_system_fonts();
        options
    }

    pub fn setup_context() -> SetupContext {
        SetupContext {
            data_source: Default::default(),
            usvg_options: Default::default(),
        }
    }
}

pub fn render_text_1x(c: &mut Criterion) {
    c.bench_function("render_text_1x", |b| {
        let measurred_config = mock::config();
        let usvg_options = mock::usvg_options();
        let mut target = Pixmap::new(1920, 1080).unwrap();
        let rect = Rect::from_xywh(0, 0, target.width() as i32, target.height() as i32);

        let component = mock::text();
        let mut widget = Widget {
            x: HorizontalPosition::Left(Length::Pixel(0)),
            y: VerticalPosition::Top(Length::Pixel(0)),
            component,
        };

        let mut setup_context = mock::setup_context();
        widget.setup(&mut setup_context).unwrap();

        b.iter(move || {
            widget
                .render(
                    &measurred_config,
                    &usvg_options,
                    &mut target,
                    rect.clone(),
                    1.0,
                )
                .unwrap();
        })
    });
}

pub fn render_text_2x(c: &mut Criterion) {
    c.bench_function("render_text_2x", |b| {
        let measurred_config = mock::config();
        let usvg_options = mock::usvg_options();
        let mut target = Pixmap::new(1920, 1080).unwrap();
        let rect = Rect::from_xywh(0, 0, target.width() as i32, target.height() as i32);

        let component = mock::text();
        let mut widget = Widget {
            x: HorizontalPosition::Left(Length::Pixel(0)),
            y: VerticalPosition::Top(Length::Pixel(0)),
            component,
        };

        let mut setup_context = mock::setup_context();
        widget.setup(&mut setup_context).unwrap();

        b.iter(move || {
            widget
                .render(
                    &measurred_config,
                    &usvg_options,
                    &mut target,
                    rect.clone(),
                    2.0,
                )
                .unwrap();
        })
    });
}

pub fn render_text_3x(c: &mut Criterion) {
    c.bench_function("render_text_3x", |b| {
        let measurred_config = mock::config();
        let usvg_options = mock::usvg_options();
        let mut target = Pixmap::new(1920, 1080).unwrap();
        let rect = Rect::from_xywh(0, 0, target.width() as i32, target.height() as i32);

        let component = mock::text();
        let mut widget = Widget {
            x: HorizontalPosition::Left(Length::Pixel(0)),
            y: VerticalPosition::Top(Length::Pixel(0)),
            component,
        };

        let mut setup_context = mock::setup_context();
        widget.setup(&mut setup_context).unwrap();

        b.iter(move || {
            widget
                .render(
                    &measurred_config,
                    &usvg_options,
                    &mut target,
                    rect.clone(),
                    3.0,
                )
                .unwrap();
        })
    });
}

criterion_group!(benches, render_text_1x, render_text_2x, render_text_3x);
criterion_main!(benches);
