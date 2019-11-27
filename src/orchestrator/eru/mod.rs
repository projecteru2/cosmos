mod core;
mod core_grpc;

use grpc::ClientStubExt;

use super::Orchestrator;
use crate::config::get_config;
use crate::model::EruContainer;
use crate::model::Node;
use core_grpc::{CoreRPC, CoreRPCClient};

pub struct Eru {
    client: core_grpc::CoreRPCClient,
}

impl Orchestrator for Eru {
    type Sandbox = EruContainer;

    fn get_node(&self, name: &String) -> Node {
        let req = core::GetNodeOptions {
            nodename: name.to_string(),
            ..Default::default()
        };
        let resp = self.client.get_node(grpc::RequestOptions::new(), req);
        let (_, node_info, _) = resp.wait().unwrap();
        Node {
            name: node_info.name,
            endpoint: node_info.endpoint,
        }
    }

    fn update_node(&self) {}

    fn deploy_container_stats(&self, container: &Self::Sandbox) {
        let conf = get_config();
        let req = core::ContainerDeployedOptions {
            id: container.get_id(),
            appname: container.get_appname(),
            entrypoint: container.get_entrypoint(),
            nodename: conf.hostname.clone(),
            data: container.to_json().into_bytes(),
            ..Default::default()
        };
        let resp = self
            .client
            .container_deployed(grpc::RequestOptions::new(), req);
        resp.wait().unwrap();
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
