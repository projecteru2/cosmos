use std::collections::HashMap;
use std::sync::Arc;

use futures::future::ok;
use futures::Future;

use super::Node;
use super::Sandbox;
use crate::libs::get_cache;
use crate::libs::HealthCache;
use crate::logging;
use shiplift::rep::ContainerDetails;
use shiplift::Docker;

struct ContainerMeta {
    id: String,
    running: bool,
    labels: HashMap<String, String>,
    networks: HashMap<String, String>,
    healthy: bool,
}

pub struct Container {
    meta: ContainerMeta,
    pid: u64,
    name: String,
    entrypoint: String,
    ident: String,
    cpu_num: f64,
    cpu_quota: f64,
    cpu_period: f64,
    memory: f64,

    docker: Arc<Docker>,
    cache: &'static HealthCache,
}

impl Sandbox for Container {
    fn started(&self) {
        if self.has_resurged() {
            self.report();
        }

        let collector = self.collect_logs();
        tokio::spawn(collector);
    }

    fn died(&self) {
        if self.has_relapsed() {
            self.report();
        }
    }

    fn report(&self) {}
}

impl Container {
    fn has_resurged(&self) -> bool {
        match self.cache.get(&self.meta.id) {
            Some(prev_healthy) => {
                if !prev_healthy && self.meta.healthy {
                    true
                } else {
                    false
                }
            }
            None => self.meta.healthy,
        }
    }

    fn has_relapsed(&self) -> bool {
        match self.cache.get(&self.meta.id) {
            Some(prev_healthy) => {
                if prev_healthy && !self.meta.healthy {
                    true
                } else {
                    false
                }
            }
            None => !self.meta.healthy,
        }
    }

    fn collect_logs(&self) -> impl Future<Item = (), Error = ()> {
        ok::<(), ()>(())
    }
}

impl Container {
    pub fn new(id: String, docker: Arc<Docker>) -> Self {
        let details = docker.containers().get(&id).inspect().wait().unwrap();
        let (name, entrypoint, ident) = parse_container_name(&details).unwrap();

        let mut container = Container {
            meta: ContainerMeta {
                id: id,
                running: (&details).state.running,
                labels: (&details).config.labels.as_ref().unwrap().clone(),
                networks: parse_container_networks(&details).unwrap(),
                healthy: false,
            },
            pid: (&details).state.pid,
            name,
            entrypoint,
            ident,
            cpu_num: 0.0,
            cpu_quota: 0.0,
            cpu_period: 0.0,
            memory: 0.0,

            docker: docker.clone(),
            cache: get_cache(),
        };
        container.post_init(details);
        container
    }

    fn post_init(&mut self, details: ContainerDetails) {
        self.check_health()
    }

    fn check_check(&mut self) {
        self.meta.healthy = false;
    }
}

fn parse_container_name(details: &ContainerDetails) -> Option<(String, String, String)> {
    let mut infos: Vec<_> = details
        .name
        .rsplitn(3, '_')
        .map(|s| s.to_string())
        .collect();
    infos.reverse();
    if infos.len() < 3 {
        None
    } else {
        Some((infos[0].clone(), infos[1].clone(), infos[2].clone()))
    }
}

fn parse_container_networks(details: &ContainerDetails) -> Option<HashMap<String, String>> {
    if !details.state.running {
        return None;
    }

    let mut networks = HashMap::new();
    for (network, entry) in &details.network_settings.networks {
        logging::debug(&format!(
            "parsing network {}: {}",
            network, entry.ip_address
        ));
        match network.as_str() {
            "host" => {
                let node = Node::get_current();
                networks.insert("host".to_string(), node.ip_address());
            }
            _ => {
                networks.insert(network.to_string(), entry.ip_address.clone());
            }
        };
    }

    Some(networks)
}
