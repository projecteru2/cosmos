mod eru;

use crate::model::Node;
use crate::model::{EruContainer, Sandbox};

use eru::Eru;

pub trait Orchestrator {
    type Sandbox: Sandbox;

    fn get_node(&self, name: &String) -> Node;
    fn update_node(&self);
    fn deploy_container_stats(&self, sandbox: &Self::Sandbox);
}

pub fn get_orchestrator() -> Box<dyn Orchestrator<Sandbox = EruContainer>> {
    Box::new(Eru::new())
}
