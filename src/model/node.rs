use regex::Regex;

use crate::config::get_config;
use crate::orchestrator::get_orchestrator;

pub struct Node {
    pub name: String,
    pub endpoint: String,
}

impl Node {
    pub fn get_current() -> Self {
        let orche = get_orchestrator();
        let conf = get_config();
        orche.get_node(&conf.hostname)
    }

    pub fn ip_address(&self) -> String {
        let re = Regex::new(r"tcp://(?P<ip>[^:]+):(?P<port>\d+)").unwrap();
        let caps = re.captures(&self.endpoint).unwrap();
        caps.name("ip").unwrap().as_str().to_string()
    }
}
