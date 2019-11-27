mod agent;
mod config;
mod libs;
mod logging;
mod model;
mod orchestrator;

use futures::future::lazy;

use agent::{app::get_container_app, Agent};

fn main() {
    config::init();
    logging::init();
    logging::debug(&format!("{:#?}", config::get_config()));

    tokio::run(lazy(|| {
        let app = get_container_app();
        let mut agent = Agent::new(app);
        agent.start();
        Ok(())
    }));
    logging::info("exited");
}
