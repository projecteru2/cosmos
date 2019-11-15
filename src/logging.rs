use slog::*;
use slog_term::PlainSyncDecorator;

use crate::config::Config;

static mut LOG: Option<Log> = None;

pub struct Log {
    root: Logger,
}

impl Log {
    fn init(level: Level) {
        let plain = PlainSyncDecorator::new(std::io::stdout());
        let drain = slog_term::FullFormat::new(plain)
            .build()
            .filter_level(level)
            .fuse();
        let root = Logger::root(drain, o!());
        unsafe {
            LOG = Some(Log::new(root));
        }
    }

    fn new(root: Logger) -> Self {
        Log { root: root }
    }

    pub fn get(module: &'static str) -> Logger {
        unsafe {
            LOG.as_ref()
                .expect("log not init")
                .root
                .new(o!("module" => module))
        }
    }
}

pub fn init() {
    let conf = Config::get();
    let level = match conf.log_level.to_lowercase().as_str() {
        "panic" | "fatal" => Level::Critical,
        "error" => Level::Error,
        "warn" | "warning" => Level::Warning,
        "info" => Level::Info,
        "debug" => Level::Debug,
        "trace" => Level::Trace,
        _ => panic!("invalid log level"),
    };

    Log::init(level);
}
