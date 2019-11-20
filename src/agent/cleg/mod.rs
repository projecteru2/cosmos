use std::sync::Arc;

use futures::Future;
use futures::Stream;

use super::CosmosApp;
use crate::logging;

pub struct Cleg<T: CosmosApp + 'static> {
    pub app: &'static T,
}

impl<T: CosmosApp> Cleg<T> {
    pub fn new(app: &'static T) -> Self {
        Cleg { app }
    }

    pub fn start(&mut self) {
        let app = Arc::new(self.app);
        let watcher = self
            .app
            .watch()
            .for_each(move |e| {
                app.clone().handle_events(e);
                Ok(())
            })
            .map_err(|e| {
                logging::error(&format!("error: {:#?}", e));
            });

        logging::info("docker event watcher starts");
        tokio::spawn(watcher);
    }
}
