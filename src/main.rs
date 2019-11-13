#[macro_use]
extern crate clap;

mod config;
use config::Config;

fn main() {
    config::init();
    let conf = Config::get();

    println!("config: {:#?}", conf);
}
