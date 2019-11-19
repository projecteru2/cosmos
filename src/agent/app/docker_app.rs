use futures::Stream;

use shiplift::builder::{EventFilter, EventFilterType, EventsOptions};
use shiplift::errors::Error as DockerError;
use shiplift::rep::Event as DockerEvent;
use shiplift::Docker;

use super::CosmosApp;
use crate::logging;

pub struct ContainerApp {
    docker: Docker,
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
            docker: Docker::new(),
        }
    }
}

impl CosmosApp for ContainerApp {
    type Event = DockerEvent;
    type Error = DockerError;

    fn handle_start_event(&self, event: Self::Event) {
        logging::debug(&format!("handling start event: {:#?}", event));
    }

    fn handle_die_event(&self, event: Self::Event) {
        logging::debug(&format!("handling die event: {:#?}", event));
    }

    fn watch(&self) -> Box<dyn Stream<Item = Self::Event, Error = Self::Error> + Send> {
        let event_filters = vec![EventFilter::Type(EventFilterType::Container)];
        let opts = EventsOptions::builder().filter(event_filters).build();
        Box::new(self.docker.events(&opts))
    }
}
