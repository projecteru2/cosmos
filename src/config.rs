use std::{fs, sync::Once};

use clap::{load_yaml, App, ArgMatches};
use hostname::get_hostname;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub core: String,
    #[serde(default = "empty_string")]
    pub log_level: String,
    #[serde(default = "empty_string")]
    pub pidfile: String,
    #[serde(default)]
    pub docker: DockerConfig,
    #[serde(default)]
    pub metrics: MetricsConfig,
    #[serde(default)]
    pub api: ApiConfig,
    #[serde(default)]
    pub log: LogConfig,
    #[serde(default = "zero_i32")]
    pub health_check_interval: i32,
    #[serde(default = "zero_i32")]
    pub health_check_timeout: i32,
    #[serde(default = "empty_string")]
    pub hostname: String,
    #[serde(default)]
    pub auth: AuthConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DockerConfig {
    #[serde(default = "empty_string")]
    pub endpoint: String,
}

impl Default for DockerConfig {
    fn default() -> Self {
        DockerConfig {
            endpoint: empty_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricsConfig {
    #[serde(default = "zero_i32")]
    pub step: i32,
    #[serde(default = "empty_vec_string")]
    pub transfers: Vec<String>,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        MetricsConfig {
            step: zero_i32(),
            transfers: empty_vec_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiConfig {
    pub addr: String,
}

impl Default for ApiConfig {
    fn default() -> Self {
        ApiConfig {
            addr: empty_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogConfig {
    #[serde(default = "empty_vec_string")]
    pub forwards: Vec<String>,
    #[serde(default = "false_bool")]
    pub stdout: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        LogConfig {
            forwards: empty_vec_string(),
            stdout: false_bool(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    #[serde(default = "empty_string")]
    pub username: String,
    #[serde(default = "empty_string")]
    pub password: String,
}

impl Default for AuthConfig {
    fn default() -> Self {
        AuthConfig {
            username: empty_string(),
            password: empty_string(),
        }
    }
}

fn empty_string() -> String {
    "".to_string()
}

fn zero_i32() -> i32 {
    0
}

fn empty_vec_string() -> Vec<String> {
    vec![]
}

fn false_bool() -> bool {
    false
}

static INIT_CONFIG: Once = Once::new();
static mut CONFIG: Option<Config> = None;

impl Config {
    fn from_matches(matches: ArgMatches) {
        let mut called = false;

        unsafe {
            INIT_CONFIG.call_once(|| {
                called = true;
                CONFIG = Some(Config::new(matches));
            });
        }
        if !called {
            panic!("init config more than once");
        }
    }

    fn get() -> &'static Config {
        unsafe { CONFIG.as_ref().expect("config not init") }
    }

    fn new(matches: ArgMatches) -> Config {
        let filename = matches.value_of("config").unwrap();
        let content = fs::read_to_string(filename).unwrap();
        let mut conf: Config = serde_yaml::from_str(&content).unwrap();
        conf.update(matches);
        conf
    }

    fn update(&mut self, matches: ArgMatches) {
        if let Some(core) = matches.value_of("core_endpoint") {
            self.core = core.to_string();
        } else if self.core == "" {
            panic!("Config.core not specified");
        }

        if let Some(log_level) = matches.value_of("log_level") {
            self.log_level = log_level.to_string();
        } else if self.log_level == "" {
            self.log_level = "INFO".to_string();
        }

        if let Some(pidfile) = matches.value_of("pidfile") {
            self.pidfile = pidfile.to_string();
        } else if self.pidfile == "" {
            self.pidfile = "/tmp/agent.pid".to_string();
        }

        if let Some(docker_endpoint) = matches.value_of("docker_endpoint") {
            self.docker.endpoint = docker_endpoint.to_string();
        } else if self.docker.endpoint == "" {
            self.docker.endpoint = "unix:///var/run/docker.sock".to_string();
        }

        if let Some(metrics_step) = matches.value_of("metrics_step") {
            self.metrics.step = metrics_step.to_string().parse().unwrap();
        } else if self.metrics.step == 0 {
            self.metrics.step = 10;
        }

        if let Some(metrics_transfers) = matches.values_of("metrics_transfers") {
            self.metrics.transfers = metrics_transfers.map(|s| s.to_string()).collect();
        }

        if let Some(api_addr) = matches.value_of("api_addr") {
            self.api.addr = api_addr.to_string();
        } else if self.api.addr == "" {
            panic!("Config.api.addr not specified");
        }

        if let Some(forwards) = matches.values_of("log_forwards") {
            self.log.forwards = forwards.map(|s| s.to_string()).collect();
        } else if self.log.forwards.len() == 0 {
            self.log.forwards.push("__discard__".to_string());
        }

        if let Some(stdout) = matches.value_of("log_stdout") {
            self.log.stdout = stdout == "yes";
        }

        if let Some(health_check_timeout) = matches.value_of("health_check_timeout") {
            self.health_check_timeout = health_check_timeout.parse().unwrap();
        } else if self.health_check_timeout == 0 {
            self.health_check_timeout = 3;
        }

        if let Some(health_check_interval) = matches.value_of("health_check_interval") {
            self.health_check_interval = health_check_interval.parse().unwrap();
        } else if self.health_check_interval == 0 {
            self.health_check_interval = 10;
        }

        if let Some(host) = matches.value_of("hostname") {
            self.hostname = host.to_string();
        } else if self.hostname == "" {
            self.hostname = get_hostname().unwrap();
        }

        if let Some(username) = matches.value_of("core_user") {
            self.auth.username = username.to_string();
        }

        if let Some(password) = matches.value_of("core_password") {
            self.auth.password = password.to_string();
        }
    }
}

pub fn init() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    Config::from_matches(matches);
}

pub fn get_config() -> &'static Config {
    Config::get()
}
