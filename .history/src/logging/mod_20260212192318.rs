use tracing_appender::rolling;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

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
