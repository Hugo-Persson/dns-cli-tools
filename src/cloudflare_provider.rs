use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::path::PathBuf;

use cloudflare::endpoints::dns::{DnsContent, DnsRecord};
use cloudflare::endpoints::{dns, zone};
use cloudflare::framework::auth::Credentials;
use cloudflare::framework::{Environment, HttpApiClientConfig};

use inquire::{prompt_text, Confirm};
use reqwest::Client;
use serde::{Deserialize, Serialize};
const CLOUDFLARE_API_URL: &str = "https://api.cloudflare.com/client/v4";

use crate::config::{Config, Domain, CONFIG_SINGLETON};
use crate::dns_provider::DnsProvider;

use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZoneResponse {
    pub result: Vec<Zone>,
    pub success: bool,
    pub errors: Vec<Value>,
    pub messages: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Zone {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DNSListResponse {
    pub result: Vec<Result>,
    pub success: bool,
    pub errors: Vec<Value>,
    pub messages: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: String,
    pub proxiable: bool,
    pub proxied: bool,
    pub ttl: i64,
    pub locked: bool,
    pub comment: Value,
    pub tags: Vec<Value>,
    #[serde(rename = "created_on")]
    pub created_on: String,
    #[serde(rename = "modified_on")]
    pub modified_on: String,
    pub priority: Option<i64>,
}

pub struct CloudflareProvider {
    client: Client,
    config: Config,
}
impl CloudflareProvider {
    pub async fn new() -> Self {
        let client = Client::new();
        let mut config = CONFIG_SINGLETON.lock().await.get();
        if config.cloudflare_config.api_token.is_empty() {
            let token =
                prompt_text("Please enter your cloudflare api token").expect("Could not get token");
            config.cloudflare_config.api_token = token;
            CONFIG_SINGLETON.lock().await.save(config.clone());
        }

        Self { client, config }
    }
    async fn sync_zones(&mut self) {
        let response: ZoneResponse = self
            .client
            .get(format!("{}/zones", CLOUDFLARE_API_URL))
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", self.config.cloudflare_config.api_token),
            )
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        response.result.iter().for_each(|zone| {
            if let Some(domain) = self.config.cloudflare_config.domains.get_mut(&zone.id) {
                domain.domain = zone.name.clone();
            } else {
                let ans =
                    Confirm::new(format!("Do you want to add the domain: {}", zone.name).as_str())
                        .with_default(false)
                        .prompt();
                match ans {
                    Ok(true) => {
                        println!("Added domain: {}", zone.name);
                        self.config.cloudflare_config.domains.insert(
                            zone.id.clone(),
                            Domain {
                                domain: zone.name.clone(),
                                records: vec![],
                            },
                        );
                    }
                    _ => {}
                }
            }
        });
        CONFIG_SINGLETON.lock().await.save(self.config.clone());
    }
}
impl DnsProvider for CloudflareProvider {
    async fn set_sub_domain(&self, record: &crate::config::Record) {
        todo!()
    }

    async fn remove_sub_domain(&self, record: &crate::config::Record) {
        todo!()
    }

    fn change_ip(&self, ip: &String) -> Self {
        todo!()
    }

    async fn import(&mut self) {
        self.sync_zones().await;
        for (id, domain) in self.config.cloudflare_config.domains.iter() {
            let response = self
                .client
                .get(format!("{}/zones/{}/dns_records", CLOUDFLARE_API_URL, id))
                .header(
                    reqwest::header::AUTHORIZATION,
                    format!("Bearer {}", self.config.cloudflare_config.api_token),
                )
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            println!("{}", response);
        }
    }
}
