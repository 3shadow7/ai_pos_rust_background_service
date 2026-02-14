use tracing_appender::rolling;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, Duration};

pub fn cleanup_old_logs(days_retention: u64) {
    let log_dir = "logs";
    let retention_duration = Duration::from_secs(days_retention * 24 * 60 * 60);
    let now = SystemTime::now();

    if let Ok(entries) = fs::read_dir(log_dir) {
        let mut count = 0;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Ok(metadata) = fs::metadata(&path) {
                    if let Ok(modified) = metadata.modified() {
                        if let Ok(age) = now.duration_since(modified) {
                            if age > retention_duration {
                                if let Err(e) = fs::remove_file(&path) {
                                    eprintln!("Failed to delete old log {:?}: {}", path, e);
                                } else {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        if count > 0 {
            println!("Cleaned up {} old log files (older than {} days).", count, days_retention);
        }
    }
}

pub fn init_logging(log_level: &str) {
    let file_appender = rolling::daily("logs", "pos_service.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // We need to keep _guard alive, but for simplicity in this setup we might leak it or return it.
    // However, tracing_appender docs say the guard must be assigned to a variable.
    // Since this is global init, we can't easily return it to main if we want to hide complexity.
    // Actually, for a service, we usually init in main and keep the guard.
    // So I will change this to return the guard.
}

pub fn get_subscriber(log_level: &str) -> (impl tracing::Subscriber, tracing_appender::non_blocking::WorkerGuard) {
    let file_appender = rolling::daily("logs", "pos_service.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(log_level));

    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(fmt::Layer::new().with_writer(std::io::stdout))
        .with(fmt::Layer::new().with_writer(non_blocking).with_ansi(false));

    (subscriber, guard)
}
