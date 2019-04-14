extern crate clap;
extern crate reqwest;
extern crate serde_derive;
extern crate toml;
extern crate serde;

use clap::{Arg, App};
use serde_derive::{Deserialize};
use std::fs;
use std::error::Error;
use std::str;

#[derive(Deserialize)]
struct Config {
    api_token: String,
    soa_email: String,
    domain: String,
}

fn read_config(config_file: &str) -> Result<Config, String> {
    let file_data = match fs::read(&config_file) {
        Ok(f) => f,
        Err(f) => {
            return Err(format!("Failed to read config file '{}', {}", config_file, f))
        }
    };

    match toml::from_slice(&file_data) {
        Ok(f) => Ok(f),
        Err(f) => Err(format!("There was a problem parsing config file, {}", f)),
    }
}

fn run()-> Result<(), Box<Error>> {
    let matches = App::new("lindyndns")
                    .version("0.0.0")
                    .author("John Ramsden <johnramsden@riseup.net>")
                    .about("Linode Dynamic DNS Client")
                    .arg(Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .value_name("FILE")
                        .help("Sets a custom config file")
                        .takes_value(true))
                    .get_matches();

    let config_file = matches.value_of("config").unwrap_or("config.toml");
    let config = read_config(config_file)?;

    print!("Getting ip for domain '{}'\n", config.domain);

    Ok(())
}

fn main() -> Result<(), Box<std::error::Error>> {
    run()
}
