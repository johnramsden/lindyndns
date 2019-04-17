extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use http::Method;
use reqwest::Response;

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
    pub port: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_sec: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<String>>,
}

// trait Paginated<T> {
//     fn extend_if_more(&self, page: u8, response: Response) -> Result<(bool, Vec<T>), Box<Error>>;
// }

// impl Paginated<Domain> for DomainsResponse {
//     fn extend_if_more(&self, page: u8, response: Response) -> Result<(bool, Vec<Domain>), Box<Error>> {
//         let mut more = false;
//         let domain_resp: DomainsResponse = response.json()?;
//         if page < self.pages {
//             more = true;
//         }
//         Ok((more, domain_resp.data))
//     }
// }

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
            let dr: DomainsResponse = self.get("/domains", Some(&[("page", &page.to_string())]))?.json()?;

            println!("DomainsResponse {:?}", dr);
            println!("Page {} of {}", page, dr.pages);

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

        let domain_created: Domain = self.http_client.request(
                Method::POST, &format!("{}{}", self.api_url, "/domains"))
                    .bearer_auth(&self.api_token)
                        .json(domain)
                        .send()?
                        .json()?;

        Ok(domain_created)
    }

    pub fn list_records(&self, domain_id: &String) -> Result<Vec<Record>, Box<Error>> {

        let mut page = 1;
        let mut records: Vec<Record> = Vec::new();
        let req = &format!("{}{}{}{}", self.api_url, "/domains/", domain_id, "/records");
        loop {
            let rp: RecordResponse = self.get(req, Some(&[("page", &page.to_string())]))?.json()?;

            println!("Page {} of {}", page, rp.pages);

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
        println!("Request: {}", req);

        let builder = self.http_client.request(Method::GET, req).bearer_auth(&self.api_token);

        let builder_query = match query {
            Some(q) => builder.query(&Some(q)),
            None => builder,
        };

        let resp = builder_query.send()?;

        Ok(resp)
    }

}

