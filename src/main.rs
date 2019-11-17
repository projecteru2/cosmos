mod agent;
mod config;
mod logging;

fn main() {
    config::init();
    logging::init();
    logging::debug(&format!("{:#?}", config::Config::get()));

    let ag = agent::Agent::new();
    ag.start();
    ag.wait();
    logging::info("exited");
}
