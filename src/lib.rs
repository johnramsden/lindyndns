extern crate toml;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_derive::Deserialize;
use std::{fs, str, fmt};
use std::error::Error;
use std::path::PathBuf;

mod linode;
use linode::client::{Domain,Record};

#[derive(Deserialize)]
struct Config {
    api_token: String,
    soa_email: String,
    domain: String,
}

#[derive(Debug)]
pub struct MyError(pub String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred: {}", self.0)
    }
}

impl std::error::Error for MyError {}

fn find_config_from_env(env: &str, suffix: &Vec<&str>) -> Option<PathBuf> {
    match std::env::var(env) {
        Ok(val) => {
            let mut cfg_entry = vec![val.as_str()];
            cfg_entry.extend(suffix);
            let path: PathBuf = cfg_entry.iter().collect();
            if path.exists() {
                return Some(path);
            }

            None
        },
        Err(_v) => None,
    }
}

// TODO: Test
fn find_config_windows() -> Option<PathBuf> {
    let suffix = vec!["lindyndns", "config.toml"];

    let config_location = match find_config_from_env("LOCALAPPDATA", &suffix) {
        Some(val) => Some(val),
        None => find_config_from_env("APPDATA", &suffix),
    };

    config_location
}

fn find_user_config_unix() -> Option<PathBuf> {
    let suffix = vec!["lindyndns", "config.toml"];
    let config_path = match find_config_from_env("XDG_CONFIG_HOME", &suffix) {
        Some(val) => Some(val),
        None => {
            let mut cfg_suffix = vec![".config"];
            cfg_suffix.extend(&suffix);

            find_config_from_env("HOME", &cfg_suffix)
        }
    };

    config_path
}

fn check_path_exists(path: Vec<&str>) -> Option<PathBuf> {
    let path_buffer: PathBuf = path.iter().collect();
    if path_buffer.exists() {
        return Some(path_buffer);
    }
    None
}

fn find_system_config_unix() -> Option<PathBuf> {
    let default_path = vec!["/", "etc", "xdg", "lindyndns", "config.toml"];
    let config_path: Option<PathBuf> = match std::env::var("XDG_DATA_DIRS") {
        Ok(val) => {
            let directory_it = val.split(":");
            for e in directory_it {
                let p = check_path_exists(vec![e, "lindyndns", "config.toml"]);
                if p.is_some() {
                    return p;
                }
            }
            check_path_exists(default_path)
        },
        Err(_v) => check_path_exists(default_path),
    };

    config_path
}

fn find_config_unix() -> Option<PathBuf> {

    let config_path: Option<PathBuf> = match find_user_config_unix() {
        Some(val) => Some(val),
        None => find_system_config_unix(),
    };

    config_path
}

// TODO
fn find_config_macos() -> Option<PathBuf> {
    let config_location = PathBuf::new();

    Some(config_location)
}

pub fn find_config() -> Result<Option<PathBuf>, Box<Error>> {
    let config_location = if cfg!(target_os = "windows") {
        find_config_windows()
    } else if cfg!(target_os = "macos") {
        find_config_macos()
    } else {
        find_config_unix()
    };

    Ok(config_location)
}

fn read_config(config_file: &str) -> Result<Config, String> {

    // println!("{:?}", find_config());

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

fn find_domain(domains: Vec<Domain>, config: &Config) -> Option<Domain> {

    for d in domains {
        if d.domain == config.domain {
            return Some(d);
        }
    }

    None
}

fn find_record(records: Vec<Record>, record_type: &str,
               record_name: &str) -> Option<Record> {

    for r in records {
        if r.name == record_name && r.record_type == record_type {
            return Some(r);
        }
    }

    None
}

pub fn run(config_file: &str) -> Result<(), Box<Error>> {

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

    let domain = find_domain(domains, &config);
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

    let records = match domain_data.id {
        Some(id) => linode_client.list_records(&id),
        None => return Err(Box::new(MyError(
                        format!("{} '{}'.", "Missing domain id for", config.domain)))),
    }?;

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

    match find_record(records, "A", "") {
        Some(r) => {
            match linode_client.update_record(
                &record, &domain_data.id.unwrap(), &r.id.unwrap()) {
                Ok(r) => {
                    println!("{}", r.target.unwrap());
                    Ok(())
                },
                Err(_r) => Err(Box::new(MyError("Failed record update".to_string()))),
            }
        },
        None => {
            match linode_client.create_record(&record, &domain_data.id.unwrap()) {
                Ok(r) => {
                    println!("{}", r.target.unwrap());
                    Ok(())
                },
                Err(_r) => Err(Box::new(MyError("Failed to create record".to_string()))),
            }
        },
    }


}
