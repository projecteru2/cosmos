mod eru;

use crate::model::Container;
use crate::model::Node;

use eru::Eru;

pub trait Orchestrator {
    fn get_node(&self, name: &String) -> Node;
    fn update_node(&self);
    fn deploy_container_stats(&self, container: Container);
}

pub fn get_orchestrator() -> Box<dyn Orchestrator> {
    Box::new(Eru {})
}
