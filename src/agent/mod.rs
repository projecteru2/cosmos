mod app;
mod cleg;
mod server;

use app::App;
use cleg::Cleg;
use server::Server;

pub struct Agent {
    app: App,
    cleg: Cleg,
    server: Server,
}

impl Agent {
    pub fn new() -> Self {
        Agent {
            app: App::new(),
            cleg: Cleg::new(),
            server: Server::new(),
        }
    }

    pub fn start(&self) {
        self.install_signal_handlers();
        self.cleg.start();
        self.server.start();
    }

    pub fn wait(&self) -> Result<&'static str, &'static str> {
        Ok("exited")
    }

    fn install_signal_handlers(&self) {}
}
