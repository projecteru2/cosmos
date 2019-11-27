use futures::Stream;
use std::sync::Arc;

use shiplift::builder::{EventFilter, EventFilterType, EventsOptions};
use shiplift::errors::Error as DockerError;
use shiplift::rep::Event as DockerEvent;
use shiplift::Docker;

use super::CosmosApp;
use crate::logging;
use crate::model::EruContainer;
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
    type Sandbox = EruContainer;
    type Event = DockerEvent;
    type Error = DockerError;

    fn handle_events(&self, event: Self::Event) {
        logging::debug(&format!("event -> {:#?}", event));
        match event {
            DockerEvent {
                action,
                id: Some(id),
                ..
            } => {
                logging::info(&format!("{} event for container {}", action, id));
                if let Some(container) = self.get_sandbox(id) {
                    match action.as_str() {
                        "start" => {
                            container.started();
                        }

                        "die" => {
                            container.died();
                        }
                        _ => {
                            logging::info(&format!("ignore container event: {}", action));
                        }
                    }
                } else {
                    logging::info(&format!("invalid eru container"));
                };
            }
            _ => {
                logging::info(&format!("other type of event: {:#?}", event));
            }
        }
    }

    fn watch(&self) -> Box<dyn Stream<Item = Self::Event, Error = Self::Error> + Send> {
        let event_filters = vec![EventFilter::Type(EventFilterType::Container)];
        let opts = EventsOptions::builder().filter(event_filters).build();
        Box::new(self.docker.events(&opts))
    }

    fn get_sandbox(&self, id: String) -> Option<Self::Sandbox> {
        EruContainer::new(id, self.docker.clone())
    }
}
