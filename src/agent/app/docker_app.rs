use futures::Stream;
use std::sync::Arc;

use shiplift::builder::{EventFilter, EventFilterType, EventsOptions};
use shiplift::errors::Error as DockerError;
use shiplift::rep::Event as DockerEvent;
use shiplift::Docker;

use super::CosmosApp;
use crate::logging;
use crate::model::Container;
use crate::model::Sandbox;

pub struct ContainerApp {
    docker: Arc<Docker>,
}

static mut CONTAINER_APP: Option<ContainerApp> = None;

impl ContainerApp {
    pub fn get() -> &'static Self {
        unsafe {
            match CONTAINER_APP.as_ref() {
                None => Self::init(),
                _ => (),
            };
            CONTAINER_APP.as_ref().unwrap()
        }
    }

    fn init() {
        unsafe {
            CONTAINER_APP = Some(Self::new());
        }
    }

    fn new() -> Self {
        Self {
            docker: Arc::new(Docker::new()),
        }
    }
}

impl CosmosApp for ContainerApp {
    type Sandbox = Container;
    type Event = DockerEvent;
    type Error = DockerError;

    fn handle_events(&self, event: Self::Event) {
        logging::debug(&format!("event -> {:#?}", event));
        match event {
            DockerEvent {
                typ: _,
                action,
                actor: _,
                status: _,
                id: Some(id),
                from: _,
                time: _,
                time_nano: _,
            } => {
                logging::info(&format!("{} event for container {}", action, id));
                let container = self.get_sandbox(id);
                match action.as_str() {
                    "start" => {
                        container.started();
                    }

                    "die" => {
                        container.died();
                    }
                    _ => {
                        logging::info(&format!("ignore event: {}", action));
                    }
                }
            }
            _ => {
                logging::info(&format!("other events: {:#?}", event));
            }
        }
    }

    fn watch(&self) -> Box<dyn Stream<Item = Self::Event, Error = Self::Error> + Send> {
        let event_filters = vec![EventFilter::Type(EventFilterType::Container)];
        let opts = EventsOptions::builder().filter(event_filters).build();
        Box::new(self.docker.events(&opts))
    }

    fn get_sandbox(&self, id: String) -> Self::Sandbox {
        Container::new(id, self.docker.clone())
    }
}
