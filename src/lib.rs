extern crate reqwest;
extern crate serde_derive;
extern crate toml;
extern crate serde;

use serde_derive::{Deserialize};
use std::fs;
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

pub fn run(config_file: &str) -> Result<(), Box<std::error::Error>> {
    let config = read_config(config_file)?;

    Ok(())
}
