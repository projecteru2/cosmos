mod app;
mod cleg;
mod server;

use std::rc::Rc;

use app::App;
use cleg::Cleg;
use server::Server;

pub struct Agent {
    cleg: Cleg,
    server: Server,
}

impl Agent {
    pub fn new() -> Self {
        let app = Rc::new(App::new());
        let cleg = Cleg::new(Rc::clone(&app));
        let server = Server::new(Rc::clone(&app));
        Agent {
            cleg: cleg,
            server: server,
        }
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

pub trait CosmosApp {
    fn version(&self) -> String;
}
