use config::Config;
extern crate anyhow;
use slog::{o, Drain, Logger};

/// Initialise slog
///
/// If the `json_logs` environment variable is truthy it will enable a json logger
pub fn init_logging(config: &Config) -> anyhow::Result<Logger> {
    let use_json_logs = config.get_bool("json_logs").unwrap_or(false);
    if use_json_logs {
        let decorator = slog_json::Json::default(std::io::stdout()).fuse();
        let drain = slog_async::Async::new(decorator).build().fuse();
        Ok(slog::Logger::root(drain, o!()))
    } else {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).chan_size(1024).build().fuse();
        Ok(slog::Logger::root(drain, o!()))
    }
}
