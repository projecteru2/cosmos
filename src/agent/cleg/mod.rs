use core::time::Duration;
use std::sync::Arc;

use futures::future::ok;
use futures::Stream;
use tokio::prelude::Future;

use super::CosmosApp;
use crate::config::get_config;
use crate::logging;
use crate::model::Sandbox;
use crate::model::SandboxEvent;

pub struct Cleg<T: CosmosApp + 'static> {
    pub app: &'static T,
}

impl<T: CosmosApp> Cleg<T> {
    pub fn new(app: &'static T) -> Self {
        Cleg { app }
    }

    pub fn start(&mut self) {
        self.start_event_watcher();
        self.start_health_checker();
    }

    fn start_event_watcher(&mut self) {
        let app = Arc::new(self.app);
        let watcher = self
            .app
            .watch()
            .for_each(move |event| {
                tokio::spawn(
                    app.clone()
                        .get_sandbox(event.sandbox_id())
                        .map(|maybe_sandbox| match maybe_sandbox {
                            Some(sandbox) => sandbox.handle_event(event),
                            None => (),
                        })
                        .map_err(|_| ()),
                );
                Ok(())
            })
            .map_err(|err| logging::error(&format!("failed to watch docker events: {:#?}", err)));

        logging::info("docker event watcher starts");
        tokio::spawn(watcher);
    }

    fn start_health_checker(&mut self) {
        let app = Arc::new(self.app);
        let config = get_config();
        let checker = tokio::timer::Interval::new_interval(Duration::from_secs(
            config.health_check_interval as u64,
        ))
        .for_each(move |_| {
            tokio::spawn(
                app.clone()
                    .list_sandboxes()
                    .for_each(|sandbox| {
                        sandbox.report();
                        ok(())
                    })
                    .map_err(|_| ()),
            );
            ok(())
        })
        .map_err(|_| ());
        tokio::spawn(checker);
    }
}
