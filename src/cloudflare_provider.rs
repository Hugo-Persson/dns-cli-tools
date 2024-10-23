use std::collections::HashMap;

use inquire::{prompt_text, Confirm};
use reqwest::Client;
use serde::{Deserialize, Serialize};
const CLOUDFLARE_API_URL: &str = "https://api.cloudflare.com/client/v4";

use crate::config::{Config, Domain, Record, CONFIG_SINGLETON};
use crate::dns_provider::DnsProvider;
use crate::ip_handler::get_current_ip;

use serde_json::{from_str, json, Value};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZoneResponse {
    pub result: Option<Vec<Zone>>,
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
    pub result: Option<Vec<DnsRecord>>,
    pub success: bool,
    pub errors: Vec<Value>,
    pub messages: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DNSCreateResponse {
    pub result: Option<DnsRecord>,
    pub success: bool,
    pub errors: Vec<Value>,
    pub messages: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DnsRecord {
    pub id: String,
    #[serde(rename = "zone_id")]
    pub zone_id: String,
    #[serde(rename = "zone_name")]
    pub zone_name: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: String,
    pub proxiable: bool,
    pub proxied: bool,
    pub ttl: i64,
    //pub settings: Settings,
    //pub meta: Meta,
    pub comment: Value,
    pub tags: Vec<Value>,
    #[serde(rename = "created_on")]
    pub created_on: String,
    #[serde(rename = "modified_on")]
    pub modified_on: String,
}

pub struct CloudflareProvider {
    client: Client,
    config: Config,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateResponse {
    success: bool,
    errors: Vec<Value>,
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
        response
            .result
            .expect("DNS result list null")
            .iter()
            .for_each(|zone| {
                if let Some(domain) = self.config.cloudflare_config.domains.get_mut(&zone.id) {
                    domain.domain.clone_from(&zone.name);
                } else {
                    let ans = Confirm::new(
                        format!("Do you want to add the domain: {}", zone.name).as_str(),
                    )
                    .with_default(false)
                    .prompt();
                    if let Ok(true) = ans {
                        println!("Added domain: {}", zone.name);
                        self.config.cloudflare_config.domains.insert(
                            zone.id.clone(),
                            Domain {
                                domain: zone.name.clone(),
                                records: vec![],
                            },
                        );
                    }
                }
            });
        CONFIG_SINGLETON.lock().await.save(self.config.clone());
    }

    async fn update_ip(&self, ip: &str, record: &Record, zone_id: String) {
        let url = format!(
            "{}/zones/{}/dns_records/{}",
            CLOUDFLARE_API_URL, zone_id, record.id
        );
        let body = json!({
        "type": "A",
        "name": record.name,
        "content": ip,
        });
        let text_response = self
            .client
            .patch(url)
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", self.config.cloudflare_config.api_token),
            )
            .json(&body)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let response: UpdateResponse = serde_json::from_str(&text_response).unwrap();
        if !response.success {
            panic!("Error: {:#?}", response.errors)
        } else {
            println!("Updated {} to {}", record.name, ip);
        }
    }
}
impl DnsProvider for CloudflareProvider {
    async fn set_sub_domain(&self, record: &crate::config::Record, id: String) -> String {
        let url = format!("{}/zones/{}/dns_records", CLOUDFLARE_API_URL, id);
        let ip = get_current_ip().await.expect("Could not get current ip");
        let body = json!({
        "type": "A",
        "name": record.name,
        "proxied": true,
        "content": ip,
        });
        let text_response = self
            .client
            .post(url)
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", self.config.cloudflare_config.api_token),
            )
            .json(&body)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        //println!("{:#?}", text_response);
        let response: DNSCreateResponse = serde_json::from_str(&text_response)
            .map_err(|e| {
                println!("Failed to parse: {}", text_response);
                e
            })
            .unwrap();

        if response.success {
            response.result.unwrap().id.clone()
        } else {
            print!("{}", text_response);
            panic!("Error: {:#?}", response.errors)
        }
    }

    async fn remove_sub_domain(&self, record: &crate::config::Record, zone_id: String) {
        let url = format!(
            "{}/zones/{}/dns_records/{}",
            CLOUDFLARE_API_URL, zone_id, record.id
        );
        let ip = get_current_ip().await.expect("Could not get current ip");
        let body = json!({
        "type": "A",
        "name": record.name,
        "content": ip,
        });
        let text_response = self
            .client
            .delete(url)
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", self.config.cloudflare_config.api_token),
            )
            .json(&body)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        println!("{:#?}", text_response);
    }

    async fn change_ip(&self, ip: &str) {
        println!("Cloudflare: Updating records to {}", ip);
        for (id, domain) in self.config.cloudflare_config.domains.iter() {
            for record in domain.records.iter() {
                self.update_ip(ip, record, id.clone()).await;
            }
        }
    }

    async fn import(&mut self) {
        self.sync_zones().await;
        let ip = get_current_ip().await.expect("Could not get current ip");
        for (id, domain) in self.config.cloudflare_config.domains.iter_mut() {
            let response_txt = self
                .client
                .get(format!(
                    "{}/zones/{}/dns_records?content={}",
                    CLOUDFLARE_API_URL, id, ip
                ))
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

            let response: DNSListResponse = serde_json::from_str(&response_txt)
                .map_err(|err| {
                    println!("Could not parse response: {:#?}", err);
                    println!("Response: {}", response_txt);
                    err
                })
                .unwrap();
            if let Some(records) = response.result {
                for record in records.iter() {
                    if domain.records.iter().any(|r| r.id == record.id) {
                        continue;
                    }
                    if record.type_field == "A" {
                        println!("Importing {}", record.name);
                        domain.records.push(Record {
                            name: record.name.clone(),
                            id: record.id.clone(),
                            record_type: crate::config::RecordType::A,
                        });
                    }
                }
            } else {
                println!("Could not import, got errors {:#?}", response.errors);
            }
        }
        CONFIG_SINGLETON.lock().await.save(self.config.clone());
    }
}
