use crate::model::DockerEvent;
use futures::future::Future;
use futures::sink::Sink;
use futures::stream;
use futures::sync::mpsc;
use futures::Stream;
use shiplift::builder::ContainerListOptions;
use shiplift::builder::{EventFilter, EventFilterType, EventsOptions};
use shiplift::errors::Error as DockerError;
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
        Box::new(
            docker
                .events(&opts)
                .filter_map(|event| DockerEvent::new(event)),
        )
    }

    fn get_sandbox(
        &self,
        id: String,
    ) -> Box<dyn Future<Item = Option<Self::Sandbox>, Error = ()> + Send> {
        Box::new(
            EruContainer::new(id).map_err(|err| {
                logging::error(&format!("failed to create eru container: {:#?}", err))
            }),
        )
    }

    fn list_sandboxes(&self) -> Box<dyn Stream<Item = Self::Sandbox, Error = ()> + Send> {
        let (tx, rx) = mpsc::channel(1_024);
        let docker = Docker::new();
        tokio::spawn(
            docker
                .containers()
                .list(&ContainerListOptions::builder().all().build())
                .map_err(|e| logging::error(&format!("list containers failed: {:#?}", e)))
                .and_then(|containers| {
                    stream::iter_ok(containers).fold(tx, |tx, container| {
                        tx.send(container)
                            .map_err(|_| logging::error("send listed containerd failed"))
                    })
                })
                .map(|_| ()),
        );

        Box::new(
            rx.and_then(move |container| {
                EruContainer::new(container.id).map_err(|e| {
                    logging::error(&format!("failed to create eru container: {:#?}", e))
                })
            })
            .filter_map(|maybe_sandbox| maybe_sandbox),
        )
    }
}
