extern crate toml;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_derive::Deserialize;
use serde_json::Value;
use std::{fs, str, fmt};
use std::collections::HashMap;
use std::error::Error;

mod linode;
use linode::client::{Domain,Record};

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
        write!(f, "An error occurred: {}", self.0)
    }
}

impl std::error::Error for MyError {}

fn find_domain(client: &linode::client::Client, domains: Vec<Domain>,
               create_domain: &Domain, config: &Config) -> Option<Domain> {

    for d in domains {
        println!("Found: {}", &d.domain);
        if d.domain == config.domain {
            println!("Matched: {}", &d.domain);
            return Some(d);
        }
    }

    None
}

fn find_record(records: Vec<Record>, record_type: &str,
               record_name: &str) -> Option<Record> {

    for r in records {
        println!("{:?}", r);
        if r.name == record_name && r.record_type == record_type {
            return Some(r);
        }
    }

    None
}

pub fn run(config_file: &str) -> Result<(), Box<dyn Error>> {

    let config = read_config(config_file)?;

    let linode_client = linode::client::Client::new(config.api_token.clone());

    let domains = linode_client.list_domains()?;

    let create_domain = Domain {
        id: None,
        domain_type: String::from("master"),
        domain: config.domain.clone(),
        group: None,
        status: None,
        description: None,
        soa_email: Some(config.soa_email.clone()),
        retry_sec: None,
        master_ips: None,
        axfr_ips: None,
        expire_sec: None,
        refresh_sec: None,
        ttl_sec: Some(300),
        tags: None,
    };

    let domain = find_domain(&linode_client, domains, &create_domain, &config);
    let domain_data = match domain {
        Some(data) => data,
        None => {
            match linode_client.create_domain(&create_domain) {
                Ok(d) => d,
                Err(d) => return Err(Box::new(MyError(
                    format!("{} '{}' \n{}\n{}", "Couldn't find or create domain",
                    config.domain, "Create it manually or check token permissions.", d)))),
            }
        },
    };

    print!("{:?}", domain_data);

    let records = match domain_data.id {
        Some(id) => linode_client.list_records(&id),
        None => return Err(Box::new(MyError(
                        format!("{} '{}'.", "Missing domain id for", config.domain)))),
    }?;

    print!("{:?}", records);

    let record = Record {
        id: None,
        record_type: String::from("A"),
        name: String::from(""),
        target: Some(String::from("[remote_addr]")),
        priority: None,
        weight: None,
        port: None,
        service: None,
        protocol: None,
        ttl_sec: Some(300),
        tag: None,
    };

    let record_found = match find_record(records, "A", "") {
        Some(r) => linode_client.update_record(&record, &domain_data.id.unwrap(), &r.id.unwrap()),
        None => linode_client.create_record(&record, &domain_data.id.unwrap()),
    };

    print!("{:?}", record_found);

    Ok(())
}
