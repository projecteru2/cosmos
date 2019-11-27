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

    fn install_signal_handlers(&self) {}
}

pub trait CosmosApp: Sync {
    type Sandbox;
    type Event: std::fmt::Debug;
    type Error: std::fmt::Debug + Send;

    fn version(&self) -> String {
        "2019-11-04".to_string()
    }

    fn handle_events(&self, event: Self::Event);

    fn watch(&self) -> Box<dyn Stream<Item = Self::Event, Error = Self::Error> + Send>;

    fn get_sandbox(&self, id: String) -> Option<Self::Sandbox>;
}
