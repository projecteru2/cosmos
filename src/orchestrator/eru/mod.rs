use super::Orchestrator;
use crate::model::Container;
use crate::model::Node;

pub struct Eru {}

impl Orchestrator for Eru {
    fn get_node(&self, name: &String) -> Node {
        Node {
            name: "localhost".to_string(),
            endpoint: "tcp://10.22.12.87".to_string(),
        }
    }

    fn update_node(&self) {}

    fn deploy_container_stats(&self, container: Container) {}
}
