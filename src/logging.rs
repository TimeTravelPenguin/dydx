#![allow(dead_code)]

use std::{fs::File, sync::Arc};

use anyhow::Result;
use tracing::*;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_forest::{tag::NoTag, Printer};
use tracing_subscriber::{filter, fmt::format::Json, prelude::*};

pub struct LoggingGuard {
    guard_stdout: WorkerGuard,
    guard_logging: WorkerGuard,
    guard_metrics: WorkerGuard,
}

pub fn configure_logging(level: tracing::Level) -> Result<LoggingGuard> {
    // A layer that logs events to stdout
    let crate_name = env!("CARGO_PKG_NAME");

    let (non_blocking_stdout, guard_stdout) = tracing_appender::non_blocking(std::io::stdout());
    let stdout_printer = Printer::default().writer(non_blocking_stdout);

    let stdout_log = tracing_forest::ForestLayer::new(stdout_printer, NoTag);
    let std_filter = filter::Targets::new()
        .with_target(crate_name, level)
        .with_target("metrics", Level::TRACE);

    // A layer that logs events to a file
    let log_file = File::create("logging.log")?;
    let (non_blocking_logging, guard_logging) = tracing_appender::non_blocking(Arc::new(log_file));
    let debug_log = tracing_subscriber::fmt::layer()
        .with_thread_ids(true)
        .with_ansi(false)
        .with_writer(non_blocking_logging);

    // A layer that collects metrics using specific events
    let metrics_file = File::create("metrics.log")?;
    let (non_blocking_metrics, guard_metrics) =
        tracing_appender::non_blocking(Arc::new(metrics_file));

    let metrics_printer = Printer::default().writer(non_blocking_metrics);

    let metrics_log = tracing_forest::ForestLayer::new(metrics_printer, NoTag)
        .with_filter(filter::LevelFilter::DEBUG)
        .with_filter(filter::filter_fn(|metadata| {
            metadata.target().starts_with("metrics")
        }));

    // Define a filter to exclude metrics events from stdout and debug logs
    let non_metrics_filter =
        filter::filter_fn(|metadata| !metadata.target().starts_with("metrics"));

    tracing_subscriber::registry()
        .with(std_filter)
        .with(
            stdout_log
                .and_then(debug_log.json())
                .with_filter(non_metrics_filter),
        )
        .with(metrics_log)
        .init();

    info!("Logging with level: {}", level);

    Ok(LoggingGuard {
        guard_stdout,
        guard_logging,
        guard_metrics,
    })
}
