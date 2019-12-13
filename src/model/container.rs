use std::collections::HashMap;
use std::net::SocketAddr;

use futures::future::join_all;
use futures::future::{err, ok};
use futures::Future;
use hyper::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use shiplift::rep::ContainerDetails;
use shiplift::Docker;
use tokio::net::tcp::TcpStream;

use super::DockerEvent;
use super::Node;
use super::Sandbox;
use crate::config::get_config;
use crate::logging;
use crate::orchestrator::get_orchestrator;

#[derive(Default, Deserialize, Serialize, Clone, Debug)]
struct HealthCheck {
    #[serde(rename = "TCPPorts")]
    tcp_ports: Vec<String>,
    #[serde(rename = "HTTPPort")]
    http_port: String,
    #[serde(rename = "HTTPURL")]
    http_url: String,
    #[serde(rename = "HTTPCode")]
    http_code: u16,
}

#[derive(Default, Deserialize, Serialize, Debug)]
struct EruMeta {
    #[serde(rename = "Publish")]
    publish: Option<Vec<String>>,
    #[serde(rename = "HealthCheck")]
    health_check: Option<HealthCheck>,
}

#[derive(Default, Deserialize, Serialize, Debug)]
struct ContainerMeta {
    id: String,
    running: bool,
    labels: HashMap<String, String>,
    networks: HashMap<String, String>,
    healthy: bool,
    #[serde(skip_serializing)]
    eru: EruMeta,
}

#[derive(Default, Serialize)]
pub struct EruContainer {
    meta: ContainerMeta,
    pid: u64,
    name: String,
    entrypoint: String,
    ident: String,
    cpu_num: f64,
    cpu_quota: f64,
    cpu_period: f64,
    memory: u64,
    local_ip: String,
}

pub struct ContainerStatus {
    pub id: String,
    pub running: bool,
    pub healthy: bool,
    pub networks: HashMap<String, String>,
    pub extension: Vec<u8>,
    pub ttl: i64,
}

impl Sandbox for EruContainer {
    type Event = DockerEvent;

    fn report(&self) {
        let orc = get_orchestrator();
        orc.set_container_status(&self);
    }

    fn handle_event(&self, event: Self::Event) {
        match event {
            DockerEvent { ref action, id, .. } if action == "start" => {
                logging::info(&format!("start event for container {}", id));
                self.started();
            }
            DockerEvent { ref action, id, .. } if action == "die" => {
                logging::info(&format!("die event for container {}", id));
                self.died();
            }
            _ => (),
        };
    }
}

impl EruContainer {
    fn started(&self) {
        self.report();

        let collector = self.collect_logs();
        tokio::spawn(collector);
    }

    fn died(&self) {
        self.report();
    }

    fn collect_logs(&self) -> impl Future<Item = (), Error = ()> {
        ok::<(), ()>(())
    }
}

impl EruContainer {
    pub fn new(id: String) -> impl Future<Item = Option<Self>, Error = ()> {
        let docker = Docker::new();
        docker
            .containers()
            .get(&id)
            .inspect()
            .map_err(|_| {
                logging::error("failed to inspect eru container");
                ()
            })
            .and_then(|details| {
                if !Self::validate(&details) {
                    err(())
                } else {
                    ok(details)
                }
            })
            .and_then(move |details| {
                let (name, entrypoint, ident) = parse_container_name(&details);
                let (local_ip, networks) = parse_container_networks(&details);
                let container = EruContainer {
                    meta: ContainerMeta {
                        id: id,
                        running: (&details).state.running,
                        labels: (&details).config.labels.as_ref().unwrap().clone(),
                        networks,
                        ..Default::default()
                    },
                    pid: (&details).state.pid,
                    name,
                    entrypoint,
                    ident,
                    local_ip,
                    // TODO: cpu_quota, cpu_period
                    memory: (&details).host_config.memory.unwrap(),

                    ..Default::default()
                };
                Self::post_init(container, details).map(|container| Some(container))
            })
            .or_else(|_| ok(None))
    }

    fn validate(details: &ContainerDetails) -> bool {
        if details.name.rsplitn(3, '_').count() < 3 {
            logging::info(&format!(
                "container {} name not conform with eru pattern: {}",
                details.id, details.name
            ));
            false
        } else if details.config.labels.as_ref().unwrap().get("ERU") == None {
            logging::info(&format!(
                "container {} label ERU_META not found",
                details.id
            ));
            false
        } else {
            true
        }
    }

    fn post_init(
        mut container: Self,
        _details: ContainerDetails,
    ) -> impl Future<Item = Self, Error = ()> {
        // TODO: calculate cpu_num

        container.meta.eru = serde_json::from_str(&container.meta.labels["ERU_META"]).unwrap();
        crate::libs::pp(&container.meta.eru);
        container.check_health().map(|healthy| {
            container.meta.healthy = healthy;
            container
        })
    }

    fn check_health(&mut self) -> impl Future<Item = bool, Error = ()> {
        self.check_tcp_health()
            .join(self.check_http_health())
            .then(|result| match result {
                Ok((true, true)) => ok(true),
                Ok(_) => ok(false),
                Err(_) => ok(false),
            })
    }

    fn check_tcp_health(&self) -> impl Future<Item = bool, Error = ()> {
        let fut = if self.meta.eru.health_check.is_none() {
            logging::debug("no health check, tcp healthy");
            ok(true)
        } else {
            err(())
        };

        let local_ip = self.local_ip.clone();
        let health_check = self.meta.eru.health_check.clone();
        fut.and_then(|_| ok(true)).or_else(move |_| {
            let mut connections = vec![];
            for tcp_port in health_check.unwrap().tcp_ports {
                let tcp_netloc = format!("{}:{}", local_ip, tcp_port);
                logging::debug(&format!("checking tcp: {}", tcp_netloc));
                let addr = tcp_netloc.parse::<SocketAddr>().unwrap();
                connections.push(TcpStream::connect(&addr));
            }

            join_all(connections).then(|result| match result {
                Ok(_) => {
                    logging::debug("tcp connections established, tcp healthy");
                    ok(true)
                }
                Err(e) => {
                    logging::debug(&format!("tcp connections failed: {}, tcp unhealthy", e));
                    ok(false)
                }
            })
        })
    }

    fn check_http_health(&self) -> impl Future<Item = bool, Error = ()> {
        let fut = if self.meta.eru.health_check.is_none() {
            logging::debug("no healthcheak, http healthy");
            ok(true)
        } else {
            err(())
        };

        let health_check = self.meta.eru.health_check.clone();
        let local_ip = self.local_ip.clone();
        fut.and_then(|_| ok(true))
            .or_else(|_| {
                let health_check = health_check.unwrap();
                if health_check.http_url.is_empty()
                    || health_check.http_port.is_empty()
                    || health_check.http_code == 0
                {
                    logging::debug("no http healthcheak, http healthy");
                    ok(true)
                } else {
                    err(health_check)
                }
            })
            .or_else(move |health_check| {
                let expected_status = StatusCode::from_u16(health_check.http_code).unwrap();
                let http_uri = format!(
                    "http://{}:{}{}",
                    local_ip, health_check.http_port, health_check.http_url
                )
                .parse()
                .unwrap();
                logging::debug(&format!("checking http url: {}", http_uri));
                let client = Client::new();
                client.get(http_uri).then(move |result| match result {
                    Ok(response) => {
                        let status_code = response.status();
                        if status_code != expected_status {
                            logging::debug(&format!(
                                "http status code: {} != {}, http unhealthy",
                                status_code, expected_status
                            ));
                            ok(false)
                        } else {
                            logging::debug(&format!(
                                "http status code: {}, http healthy",
                                status_code
                            ));
                            ok(true)
                        }
                    }
                    Err(e) => {
                        logging::debug(&format!("http request failed: {}, http unhealthy", e));
                        ok(false)
                    }
                })
            })
    }

    pub fn status(&self) -> ContainerStatus {
        let conf = get_config();
        return ContainerStatus {
            id: self.meta.id.clone(),
            running: self.meta.running,
            healthy: self.meta.healthy,
            networks: self.meta.networks.clone(),
            extension: serde_json::to_vec(&self.meta.labels).unwrap(),
            ttl: (2 * conf.health_check_interval) as i64,
        };
    }
}

fn parse_container_name(details: &ContainerDetails) -> (String, String, String) {
    let mut infos: Vec<_> = details
        .name
        .rsplitn(3, '_')
        .map(|s| s.to_string())
        .collect();
    infos.reverse();
    (infos[0].clone(), infos[1].clone(), infos[2].clone())
}

fn parse_container_networks(details: &ContainerDetails) -> (String, HashMap<String, String>) {
    let mut local_ip = String::new();
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
                local_ip = "127.0.0.1".to_string();
            }
            _ => {
                networks.insert(network.to_string(), entry.ip_address.clone());
                local_ip = entry.ip_address.clone();
            }
        };
        break;
    }

    (local_ip, networks)
}
