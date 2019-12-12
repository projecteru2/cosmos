mod core;
mod core_grpc;

use grpc::ClientStubExt;

use super::Orchestrator;
use crate::config::get_config;
use crate::model::EruContainer;
use crate::model::Node;
use core_grpc::{CoreRPC, CoreRPCClient};

use crate::logging;

pub struct Eru {
    client: core_grpc::CoreRPCClient,
}

impl Orchestrator for Eru {
    type Sandbox = EruContainer;

    fn get_node(&self, name: &String) -> Node {
        println!("get node from eru core: {}", name);
        let req = core::GetNodeOptions {
            nodename: name.to_string(),
            ..Default::default()
        };
        let resp = self.client.get_node(grpc::RequestOptions::new(), req);
        let (_, node, _) = resp.wait().unwrap();
        Node {
            name: node.name,
            endpoint: node.endpoint,
        }
    }

    fn update_node(&self) {}

    fn set_container_status(&self, container: &Self::Sandbox) {
        let mut status = protobuf::RepeatedField::new();
        let state = container.status();
        status.push(core::ContainerStatus {
            id: state.id,
            running: state.running,
            healthy: state.healthy,
            networks: state.networks,
            extension: state.extension,
            ttl: state.ttl,
            ..Default::default()
        });
        let req = core::SetContainersStatusOptions {
            status: status,
            ..Default::default()
        };
        let resp = self
            .client
            .set_containers_status(grpc::RequestOptions::new(), req);
        match resp.wait() {
            Ok(_) => (),
            Err(e) => {
                logging::error(&format!("set container status failed: {}", e));
            }
        };
    }
}

impl Eru {
    pub fn new() -> Self {
        let conf = get_config();
        let core_info: Vec<_> = conf.core.splitn(2, ':').collect();
        let core_port = core_info[1].to_string().parse().unwrap();
        let client_conf = Default::default();
        let client = CoreRPCClient::new_plain(core_info[0], core_port, client_conf).unwrap();
        Eru { client }
    }
}
