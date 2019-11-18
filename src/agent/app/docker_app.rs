use futures::stream::{self, Stream};

use shiplift::errors::Error as DockerError;
use shiplift::rep::Event as DockerEvent;

use super::CosmosApp;
use crate::logging;

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
    type Event = DockerEvent;
    type Error = DockerError;

    fn handle_start_event(&self, event: Self::Event) {
        logging::debug(&format!("handling start event: {:#?}", event));
    }

    fn handle_die_event(&self, event: Self::Event) {
        logging::debug(&format!("handling die event: {:#?}", event));
    }

    fn watch(&self) -> Box<dyn Stream<Item = Result<Self::Event, Self::Error>> + Unpin + Send> {
        use std::collections::HashMap;
        let values = vec![Ok(DockerEvent {
            typ: "container".to_string(),
            action: "die".to_string(),
            actor: shiplift::rep::Actor {
                id: "".to_string(),
                attributes: HashMap::new(),
            },
            status: None,
            id: None,
            from: None,
            time: 0,
            time_nano: 0,
        })];
        Box::new(stream::iter(values))
    }
}
