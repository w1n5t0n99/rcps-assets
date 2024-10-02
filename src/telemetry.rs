use anyhow::{Context, Result};
use time::macros::format_description;
use tokio::task::{JoinHandle, spawn_blocking};
use tracing::{subscriber::set_global_default, Level};
use tracing_appender::{non_blocking::WorkerGuard, rolling::{RollingFileAppender, Rotation}};
use tracing_subscriber::{fmt::{time::UtcTime, writer::MakeWriterExt, Layer}, layer::SubscriberExt, EnvFilter, Registry};


pub fn init_console_subscriber(level: Level) -> Result<WorkerGuard>  {
    let filter = EnvFilter::from_default_env().add_directive(level.into());

    let log_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY) 
        .filename_suffix("log") 
        .build("./log") 
        .context("initialising rolling file appender failed")?;

    let (non_blocking_appender, log_guard) = tracing_appender::non_blocking(log_appender);

    let timer = UtcTime::new(format_description!("[year]-[month]-[day]-[hour]:[minute]:[second]"));

    let subscriber = Registry::default()
        .with(filter)
        .with(
            Layer::new()
                .with_timer(timer)
                .with_ansi(false)
                .with_writer(non_blocking_appender.and(std::io::stdout))
        );

    set_global_default(subscriber).context("failed to set subscriber")?;

    Ok(log_guard)
}

pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    spawn_blocking(move || current_span.in_scope(f))
}