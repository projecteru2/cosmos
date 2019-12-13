use futures::future::Future;
use futures::sync::{mpsc, oneshot};
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

    fn get_sandbox(&self, event: &DockerEvent) -> oneshot::Receiver<Option<Self::Sandbox>> {
        let (tx, rx) = oneshot::channel();
        tokio::spawn(
            EruContainer::new(event.id.as_ref().unwrap().clone())
                .and_then(move |maybe_container| {
                    tx.send(maybe_container).map_err(|_| {
                        logging::error("failed to send maybe_conatiner");
                        oneshot::Canceled {}
                    })
                })
                .map_err(|err| {
                    logging::error(&format!("failed to create eru container: {:#?}", err))
                })
                .map(|_| ()),
        );
        rx
    }

    fn list_sandboxes(&self) -> mpsc::Receiver<Self::Sandbox> {
        let (_, rx) = mpsc::channel(1_024);
        rx
    }
}
