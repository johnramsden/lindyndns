extern crate toml;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_derive::Deserialize;
use serde_json::Value;
use std::{fs, str, fmt};

mod linode;
use linode::client::{Client,Domain};

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

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl std::error::Error for MyError {}

pub fn run(config_file: &str) -> Result<(), Box<dyn std::error::Error>> {

    let config = read_config(config_file)?;

    let linode_client = linode::client::Client::new(config.api_token);

    let domains = linode_client.list_domains()?;

    println!("# ----------------------- #");
    let mut domain_data: Option<Domain> = None;
    for d in domains {
        if d.domain == config.domain {
            domain_data = Some(d);
        }
    }

    if domain_data.is_none() {
        return Result::Err(Box::new(MyError(format!("Couldn't find domain '{}'", config.domain))));
    }

    print!("{}", domain_data.unwrap().domain);

    Ok(())
}
