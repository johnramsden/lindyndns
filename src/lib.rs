extern crate reqwest;
extern crate toml;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_derive::Deserialize;
use serde_json::Value;
use reqwest::{RequestBuilder, Client};
use http::Method;

use std::{fs, str};

#[derive(Deserialize)]
struct Config {
    api_token: String,
    soa_email: String,
    domain: String,
}

#[derive(Deserialize, Debug)]
struct DomainData {
    id: u32,
    r#type: String,
    domain: String,
    group: String,
    status: String,
    description: String,
    soa_email: String,
    retry_sec: u32,
    master_ips: Vec<String>,
    axfr_ips: Vec<String>,
    expire_sec: u32,
    refresh_sec: u32,
    ttl_sec: u32,
    tags: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Domains {
    data: Vec<DomainData>,
    page: u8,
    pages: u8,
    results: u8,
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
    let api_url = "https://api.linode.com/v4";

    let config = read_config(config_file)?;

    let client = reqwest::Client::new();

    let domains: Domains = client.request(Method::GET, &format!("{}{}", api_url, "/domains"))
        .bearer_auth(config.api_token)
        .send()?
        .json()?;

    print!("{:?}", domains);
    // print!("{}", );

    // for e in domains["data"] {
    //     print!("{}", e);
    // }

    Ok(())
}
