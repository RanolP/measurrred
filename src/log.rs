use tracing::metadata::LevelFilter;
use tracing_subscriber::prelude::*;

pub fn initialize_tracing_logger() {
    let my_filter = tracing_subscriber::filter::filter_fn(|metadata| {
        !matches!(
            metadata.module_path(),
            Some("surf::middleware::logger::native")
        )
    });

    let file_appender = tracing_appender::rolling::daily("logs", "measurrred");
    let (file_appender, _guard1) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::registry()
        .with(LevelFilter::INFO)
        .with(my_filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_writer(file_appender),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
