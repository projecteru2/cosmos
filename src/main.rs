mod agent;
mod config;
mod logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::init();
    logging::init();
    logging::debug(&format!("{:#?}", config::Config::get()));

    let mut ag = agent::Agent::new();
    ag.start();
    ag.wait().await?;
    logging::info("exited");
    Ok(())
}
