mod agent;
mod config;
mod logging;

use agent::{app::get_container_app, Agent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::init();
    logging::init();
    logging::debug(&format!("{:#?}", config::Config::get()));

    let app = get_container_app();
    let mut agent = Agent::new(app);
    agent.start();
    agent.wait().await?;
    logging::info("exited");
    Ok(())
}
