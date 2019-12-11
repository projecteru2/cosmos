use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use futures::future::ok;
use futures::sync::oneshot;
use futures::Future;
use hyper::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use shiplift::rep::ContainerDetails;
use shiplift::Docker;
use tokio::net::tcp::TcpStream;

use super::Node;
use super::Sandbox;
use crate::config::get_config;
use crate::logging;
use crate::orchestrator::get_orchestrator;

#[derive(Default, Deserialize, Serialize)]
struct HealthCheck {
    tcp_ports: Vec<String>,
    http_port: String,
    http_url: String,
    http_code: u16,
}

#[derive(Default, Deserialize, Serialize)]
struct EruMeta {
    #[serde(rename = "Publish")]
    publish: Option<Vec<String>>,
    #[serde(rename = "HealthCheck")]
    health_check: Option<HealthCheck>,
}

#[derive(Default, Deserialize, Serialize)]
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

    #[serde(skip_serializing)]
    docker: Arc<Docker>,
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
    fn started(&self) {
        self.report();

        let collector = self.collect_logs();
        tokio::spawn(collector);
    }

    fn died(&self) {
        self.report();
    }

    fn report(&self) {
        let orc = get_orchestrator();
        orc.deploy_container_stats(&self);
    }
}

impl EruContainer {
    fn collect_logs(&self) -> impl Future<Item = (), Error = ()> {
        ok::<(), ()>(())
    }
}

impl EruContainer {
    pub fn new(id: String, docker: Arc<Docker>) -> oneshot::Receiver<Option<Self>> {
        let (tx, rx) = oneshot::channel();
        tokio::spawn(
            docker
                .containers()
                .get(&id)
                .inspect()
                .and_then(move |details| {
                    if !Self::validate(&details) {
                        logging::error(&format!("not eru container: {}", id));
                        return tx.send(None).map_err(|_| {
                            logging::error("failed to send eru container");
                            shiplift::Error::IO(io::Error::new(io::ErrorKind::Other, "oh no!"))
                        });
                    }

                    let (name, entrypoint, ident) = parse_container_name(&details);
                    let (local_ip, networks) = parse_container_networks(&details);
                    let mut container = EruContainer {
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

                        docker: docker.clone(),

                        ..Default::default()
                    };

                    container.post_init(details);
                    tx.send(Some(container)).map_err(|_| {
                        logging::error("failed to send eru container");
                        shiplift::Error::IO(io::Error::new(io::ErrorKind::Other, "oh no!"))
                    })
                })
                .map_err(|err| {
                    logging::error(&format!("failed to fetch container inspect: {}", err))
                })
                .map(|_| ()),
        );
        rx
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

    fn post_init(&mut self, _details: ContainerDetails) {
        // TODO: calculate cpu_num

        self.meta.eru = serde_json::from_str(&self.meta.labels["ERU_META"]).unwrap();
        self.check_health();
    }

    fn check_health(&mut self) {
        self.meta.healthy = self.check_tcp_health() && self.check_http_health();
    }

    fn check_tcp_health(&self) -> bool {
        if self.meta.eru.health_check.is_none() {
            return true;
        }

        let healthy = RefCell::new(false);
        for tcp_port in &self.meta.eru.health_check.as_ref().unwrap().tcp_ports {
            let tcp_netloc = format!("{}:{}", self.local_ip, tcp_port);
            let addr = tcp_netloc.parse::<SocketAddr>().unwrap();
            crate::libs::pp(&addr);
            TcpStream::connect(&addr)
                .map(|_res| {
                    logging::debug(&format!("tcp check for container {} passes", self.meta.id));
                    *healthy.borrow_mut() = true;
                })
                .map_err(|err| {
                    logging::info(&format!(
                        "tcp check for container {} fails: {}",
                        self.meta.id, err
                    ));
                    *healthy.borrow_mut() = false;
                })
                .wait()
                .unwrap();
        }
        healthy.into_inner()
    }

    fn check_http_health(&self) -> bool {
        if self.meta.eru.health_check.is_none() {
            return true;
        }

        let health_check = self.meta.eru.health_check.as_ref().unwrap();
        let healthy = RefCell::new(false);
        let expected_status = StatusCode::from_u16(health_check.http_code).unwrap();
        let http_uri = format!(
            "http://{}:{}{}",
            self.local_ip, health_check.http_port, health_check.http_url
        )
        .parse()
        .unwrap();
        let client = Client::new();
        client
            .get(http_uri)
            .map(|res| {
                let status_code = res.status();
                logging::debug(&format!(
                    "http check for container {} got status code: {}",
                    self.meta.id, &status_code
                ));
                if status_code != expected_status {
                    logging::info(&format!(
                        "http check for container {} fails: {}",
                        self.meta.id, status_code
                    ));
                    *healthy.borrow_mut() = false;
                }
            })
            .map_err(|err| {
                logging::info(&format!(
                    "http check for container {} fails: {}",
                    self.meta.id, err
                ));
                *healthy.borrow_mut() = false;
            })
            .wait()
            .unwrap();
        healthy.into_inner()
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
