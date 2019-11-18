pub mod app;
mod cleg;
mod server;

use futures::Stream;

use cleg::Cleg;
use server::HTTPServer;

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
    type Event: std::fmt::Debug;
    type Error: std::fmt::Debug;

    fn version(&self) -> String {
        "2019-11-04".to_string()
    }

    fn handle_start_event(&self, event: Self::Event);

    fn handle_die_event(&self, event: Self::Event);

    fn watch(&self) -> Box<dyn Stream<Item = Result<Self::Event, Self::Error>> + Unpin + Send>;
}
