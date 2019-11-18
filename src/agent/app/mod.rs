mod docker_app;

pub use super::CosmosApp;
pub use docker_app::ContainerApp;

pub fn get_container_app() -> &'static ContainerApp {
    ContainerApp::get()
}
