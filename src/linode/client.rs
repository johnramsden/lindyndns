extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_derive::{Deserialize, Serialize};

use http::Method;
use reqwest::Response;

use std::error::Error;

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

#[derive(Deserialize, Debug)]
struct RecordResponse {
    #[serde(default)] data: Vec<Record>,
    #[serde(default)] page: u8,
    #[serde(default)] pages: u8,
    #[serde(default)] results: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Record {
    // Required deserializing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    #[serde(rename = "type")]
    pub record_type: String,
    pub name: String,
    // Optional
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_sec: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<String>>,
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
            let dr: DomainsResponse = self.get(
                "/domains", Some(&[("page", &page.to_string())]))?.json()?;
            domains.extend(dr.data);
            if page < dr.pages {
                page += 1;
                continue;
            }
            break;
        }

        Ok(domains)
    }

    pub fn create_domain(&self, domain: &Domain) -> Result<Domain, Box<Error>> {

        let req = &format!("{}{}", self.api_url, "/domains/");

        let domain_created: Domain = self.http_client.request(Method::POST, req)
                        .bearer_auth(&self.api_token)
                        .json(domain)
                        .send()?
                        .json()?;

        Ok(domain_created)
    }

    pub fn create_record(&self, record: &Record, domain_id: &u32) -> Result<Record, Box<Error>> {

        let req = &format!("{}{}{}{}", self.api_url, "/domains/", domain_id, "/records");

        let record_created: Record = self.http_client.request(Method::POST, req)
                                .bearer_auth(&self.api_token)
                                .json(record)
                                .send()?
                                .json()?;

        Ok(record_created)
    }

    pub fn update_record(&self, record: &Record, domain_id: &u32, record_id: &u32) -> Result<Record, Box<Error>> {

        let req = &format!("{}{}{}{}{}", self.api_url, "/domains/", domain_id, "/records/", record_id);

        let record_updated: Record = self.http_client.request(Method::PUT, req)
                                .bearer_auth(&self.api_token)
                                .json(record)
                                .send()?
                                .json()?;

        Ok(record_updated)
    }

    pub fn list_records(&self, domain_id: &u32) -> Result<Vec<Record>, Box<Error>> {

        let mut page = 1;
        let mut records: Vec<Record> = Vec::new();
        let req = &format!("{}{}{}", "/domains/", domain_id, "/records");
        loop {
            let rp: RecordResponse = self.get(req, Some(&[("page", &page.to_string())]))?.json()?;

            records.extend(rp.data);
            if page < rp.pages {
                page += 1;
                continue;
            }
            break;
        }
        
        Ok(records)
    }

    fn get(&self, endpoint: &str, query: Option<&[(&str, &str)]>) -> Result<Response, Box<Error>> {

        let req = &format!("{}{}", self.api_url, endpoint);

        let builder = self.http_client.request(Method::GET, req).bearer_auth(&self.api_token);

        let builder_query = match query {
            Some(q) => builder.query(&Some(q)),
            None => builder,
        };

        let resp = builder_query.send()?;

        Ok(resp)
    }

}
