use super::CosmosApp;

pub struct Cleg<T: CosmosApp + 'static> {
    pub app: &'static T,
}

impl<T: CosmosApp> Cleg<T> {
    pub fn new(app: &'static T) -> Self {
        Cleg { app }
    }

    pub fn start(&self) {}

    pub async fn wait(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
