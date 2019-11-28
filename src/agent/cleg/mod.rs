use std::sync::Arc;

use futures::Stream;
use tokio::prelude::Future;

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
            .for_each(move |event| {
                app.clone().handle_events(event);
                Ok(())
            })
            .map_err(|err| logging::error(&format!("failed to watch docker events: {:#?}", err)));

        logging::info("docker event watcher starts");
        tokio::spawn(watcher);
    }
}
