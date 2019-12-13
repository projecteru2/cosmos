use futures::future::Future;
use futures::sync::mpsc;
use futures::Stream;
use shiplift::builder::{EventFilter, EventFilterType, EventsOptions};
use shiplift::errors::Error as DockerError;
use shiplift::rep::Event as DockerEvent;
use shiplift::Docker;

use super::CosmosApp;
use crate::logging;
use crate::model::EruContainer;

pub struct ContainerApp {}

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
        Self {}
    }
}

impl CosmosApp for ContainerApp {
    type Sandbox = EruContainer;
    type Event = DockerEvent;
    type Error = DockerError;

    fn watch(&self) -> Box<dyn Stream<Item = Self::Event, Error = Self::Error> + Send> {
        let event_filters = vec![EventFilter::Type(EventFilterType::Container)];
        let opts = EventsOptions::builder().filter(event_filters).build();
        let docker = Docker::new();
        Box::new(docker.events(&opts))
    }

    fn get_sandbox(
        &self,
        event: &DockerEvent,
    ) -> Box<dyn Future<Item = Option<Self::Sandbox>, Error = ()> + Send> {
        Box::new(
            EruContainer::new(event.id.as_ref().unwrap().clone()).map_err(|err| {
                logging::error(&format!("failed to create eru container: {:#?}", err))
            }),
        )
    }

    fn list_sandboxes(&self) -> Box<dyn Stream<Item = Self::Sandbox, Error = ()> + Send> {
        let (_, rx) = mpsc::channel(1_024);
        Box::new(rx)
    }
}
