extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use http::Method;

use std::error::Error;
use std::collections::HashMap;

pub struct Client {
    api_token: String,
    api_url: String,
    http_client: reqwest::Client,
}

#[derive(Deserialize, Debug)]
struct DomainsResponse {
    #[serde(default)] data: Vec<Domain>,
    #[serde(default)] page: u8,
    #[serde(default)] pages: u8,
    #[serde(default)] results: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Domain {
    // Required deserializing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    #[serde(rename = "type")]
    pub domain_type: String,
    pub domain: String,
    // Optional
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub soa_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_sec: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_ips: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub axfr_ips: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expire_sec: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_sec: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_sec: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl Client {
    pub fn new(api_token: String) -> Client {
        Client {
            api_token: api_token,
            api_url: String::from("https://api.linode.com/v4"),
            http_client: reqwest::Client::new(),
        }
    }

    pub fn list_domains(&self) -> Result<Vec<Domain>, Box<Error>> {
        let mut page = 1;
        let mut domains: Vec<Domain> = Vec::new();
        loop {
            let domains_page: DomainsResponse = self.http_client.request(
                    Method::GET, &format!("{}{}", self.api_url, "/domains"))
                    .bearer_auth(&self.api_token)
                        .query(&[("page", page)])
                        .send()?
                        .json()?;

            println!("Page {} of {}", page, domains_page.pages);

            domains.extend(domains_page.data);
            if page < domains_page.pages {
                page += 1;
                continue;
            }
            break;
        }

        Ok(domains)
    }

    pub fn create_domain(&self, domain: &Domain) -> Result<Domain, Box<Error>> {

        let domain_created: Domain = self.http_client.request(
                Method::POST, &format!("{}{}", self.api_url, "/domains"))
                    .bearer_auth(&self.api_token)
                        .json(domain)
                        .send()?
                        .json()?;

        Ok(domain_created)
    }

}

