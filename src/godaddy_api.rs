use reqwest::Client;
use serde_json::json;

use crate::config::{Record, RecordType};

pub struct GoDaddyAPI {
    api_key: String,
    secret: String,
    domain: String,
    ip: String,
    client: Client,
}

impl GoDaddyAPI {
    pub fn new(api_key: String, secret: String, domain: String, ip: String) -> GoDaddyAPI {
        GoDaddyAPI {
            api_key,
            secret,
            domain,
            ip,
            client: Client::new(),
        }
    }
    pub async fn put_sub_domain(&self, record: &Record) -> () {
        let url = format!(
            "https://api.godaddy.com/v1/domains/{}/records/{}/{}",
            self.domain,
            record.record_type.to_string(),
            record.name
        );

        let body =
            vec![json!({"data": self.ip, "ttl": 600, "type": record.record_type.to_string()})];
        let resp = self
            .client
            .put(&url)
            .header(
                "authorization",
                format!("sso-key {}:{}", self.api_key, self.secret),
            )
            .json(&body)
            .send()
            .await
            .unwrap()
            .text()
            .await;

        println!("Response: {:?}", resp);
    }
}
