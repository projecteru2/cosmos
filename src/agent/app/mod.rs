use super::CosmosApp;
pub struct App {}

impl App {
    pub fn new() -> impl CosmosApp {
        App {}
    }
}

impl CosmosApp for App {
    fn version(&self) -> String {
        "2019-11-04".to_string()
    }
}
