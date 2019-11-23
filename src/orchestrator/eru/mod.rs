mod core;
mod core_grpc;

use grpc::ClientStubExt;

use super::Orchestrator;
use crate::config::Config;
use crate::model::Container;
use crate::model::Node;
use core_grpc::{CoreRPC, CoreRPCClient};

pub struct Eru {
    client: core_grpc::CoreRPCClient,
}

impl Orchestrator for Eru {
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

    fn deploy_container_stats(&self, container: Container) {}
}

impl Eru {
    pub fn new() -> Self {
        let conf = Config::get();
        let core_info: Vec<_> = conf.core.splitn(2, ':').collect();
        let core_port = core_info[1].to_string().parse().unwrap();
        let client_conf = Default::default();
        let client = CoreRPCClient::new_plain(core_info[0], core_port, client_conf).unwrap();
        Eru { client }
    }
}
