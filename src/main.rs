mod config;
mod logging;

use config::Config;

use slog::{debug, error, info};

fn main() {
    config::init();
    logging::init();

    let log = logging::Log::get("main");
    error!(log, "{:#?}", Config::get());
    info!(log, "this is log");
    debug!(log, "debug");
}
