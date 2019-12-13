pub mod app;
mod cleg;
mod server;

use futures::sync::{mpsc, oneshot};
use futures::Stream;

use crate::model::Sandbox;
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
    type Sandbox: Sandbox<Event = Self::Event> + Send;
    type Event: std::fmt::Debug + Send;
    type Error: std::fmt::Debug + Send;

    fn version(&self) -> String {
        "2019-11-04".to_string()
    }

    fn watch(&self) -> Box<dyn Stream<Item = Self::Event, Error = Self::Error> + Send>;

    fn get_sandbox(&self, event: &Self::Event) -> oneshot::Receiver<Option<Self::Sandbox>>;

    fn list_sandboxes(&self) -> mpsc::Receiver<Self::Sandbox>;
}
