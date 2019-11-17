mod app;
mod cleg;
mod server;

//use std::sync::{Arc, Mutex};

pub use app::App;
pub use cleg::Cleg;
pub use server::HTTPServer;

pub struct Agent<T: CosmosApp + 'static> {
    cleg: Cleg<T>,
    server: HTTPServer<T>,
}

impl<T: CosmosApp + 'static> Agent<T> {
    pub fn new(app: &'static T) -> Self {
        let cleg = Cleg::new(app);
        let server = HTTPServer::new(app);
        Agent { cleg, server }
    }

    pub fn start(&mut self) {
        self.install_signal_handlers();
        self.cleg.start();
        self.server.start();
    }

    pub async fn wait(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.server.wait().await?;
        self.cleg.wait().await?;
        Ok(())
    }

    fn install_signal_handlers(&self) {}
}

pub trait CosmosApp: Sync {
    fn version(&self) -> String;
}
