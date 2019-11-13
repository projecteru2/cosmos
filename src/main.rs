#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    println!(
        "config: {}",
        matches.value_of("config").unwrap_or("/etc/eru/agent.yaml")
    );
    println!(
        "health_check_interval: {}",
        matches.value_of("health_check_interval").unwrap()
    );
    if let Some("yes") = matches.value_of("log_stdout") {
        println!("log stdout: yes!");
    } else {
        println!("log stdout: false");
    }
}
