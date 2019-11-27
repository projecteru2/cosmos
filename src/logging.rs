use slog;
use slog::Drain;
use slog_term::PlainSyncDecorator;

use crate::config::get_config;

static mut LOG: Option<Log> = None;

struct Log {
    root: slog::Logger,
}

impl Log {
    fn init(level: slog::Level) {
        let plain = PlainSyncDecorator::new(std::io::stdout());
        let drain = slog_term::FullFormat::new(plain)
            .build()
            .filter_level(level)
            .fuse();
        let root = slog::Logger::root(drain, slog::o!("app" => "cosmos"));
        unsafe {
            LOG = Some(Log::new(root));
        }
    }

    fn new(root: slog::Logger) -> Self {
        Log { root: root }
    }

    fn get() -> slog::Logger {
        unsafe { LOG.as_ref().expect("log not init").root.new(slog::o!()) }
    }
}

pub fn init() {
    let conf = get_config();
    let level = match conf.log_level.to_lowercase().as_str() {
        "panic" | "fatal" => slog::Level::Critical,
        "error" => slog::Level::Error,
        "warn" | "warning" => slog::Level::Warning,
        "info" => slog::Level::Info,
        "debug" => slog::Level::Debug,
        "trace" => slog::Level::Trace,
        _ => panic!("invalid log level"),
    };

    Log::init(level);
}

#[allow(dead_code)]
pub fn info(s: &str) {
    slog::info!(Log::get(), "{}", s);
}

#[allow(dead_code)]
pub fn debug(s: &str) {
    slog::debug!(Log::get(), "{}", s);
}

#[allow(dead_code)]
pub fn error(s: &str) {
    slog::error!(Log::get(), "{}", s);
}
