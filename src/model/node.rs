use regex::Regex;

use crate::config;
use crate::orchestrator::get_orchestrator;

pub struct Node {
    pub name: String,
    pub endpoint: String,
}

impl Node {
    pub fn get_current() -> Self {
        let orche = get_orchestrator();
        let conf = config::Config::get();
        orche.get_node(&conf.hostname)
    }

    pub fn ip_address(&self) -> String {
        let re = Regex::new(r"tcp://(?P<ip>[^:]+):(?P<port>\d+)").unwrap();
        let caps = re.captures(&self.endpoint).unwrap();
        caps.name("ip").unwrap().as_str().to_string()
    }
}
