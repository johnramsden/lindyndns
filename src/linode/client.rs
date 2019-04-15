extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_derive::Deserialize;
use serde_json::Value;

use http::Method;

pub struct Client {
    api_token: String,
    api_url: String,
    http_client: reqwest::Client,
}

#[derive(Deserialize, Debug)]
struct DomainsResponse {
    data: Vec<Domain>,
    page: u8,
    pages: u8,
    results: u8,
}

#[derive(Deserialize, Debug)]
pub struct Domain {
    // Required
    pub id: u32,
    #[serde(rename = "type")]
    pub domain_type: String,
    pub domain: String,
    // Optional
    #[serde(default)] pub group: String,
    #[serde(default)] pub status: String,
    #[serde(default)] pub description: String,
    #[serde(default)] pub soa_email: String,
    #[serde(default)] pub retry_sec: u32,
    #[serde(default)] pub master_ips: Vec<String>,
    #[serde(default)] pub axfr_ips: Vec<String>,
    #[serde(default)] pub expire_sec: u32,
    #[serde(default)] pub refresh_sec: u32,
    #[serde(default)] pub ttl_sec: u32,
    #[serde(default)] pub tags: Vec<String>,
}

impl Client {
    pub fn new(api_token: String) -> Client {
        Client {
            api_token: api_token,
            api_url: String::from("https://api.linode.com/v4"),
            http_client: reqwest::Client::new(),
        }
    }

    pub fn list_domains(&self) -> Result<Vec<Domain>, Box<std::error::Error>> {
        let domains: DomainsResponse = self.http_client.request(
            Method::GET, &format!("{}{}", self.api_url, "/domains"))
                    .bearer_auth(&self.api_token)
                    .send()?
                    .json()?;

        Ok(domains.data)

        // match domains {
        //     Err(d) => Err(format!("Failed to list domains")),
        //     Ok(d) => Ok(d.data),
        // }
    }

}

