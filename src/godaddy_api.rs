use std::error::Error;

use reqwest::Client;
use serde::Serialize;
use serde_json::json;
use crate::dns_provider::DnsProvider;

use crate::config::Record;

pub struct GoDaddyAPI {
    api_key: String,
    secret: String,
    domain: String,
    ip: String,
    client: Client,
    debug: bool,
}
impl GoDaddyAPI{
    pub fn new(
        api_key: String,
        secret: String,
        domain: String,
        ip: String,
        debug: bool,
    ) -> GoDaddyAPI {
        GoDaddyAPI {
            api_key,
            secret,
            domain,
            ip,
            client: Client::new(),
            debug,
        }
    }

    async fn send_request<T>(
        client: &Client,
        url: String,
        body: T,
        api_key: &String,
        secret: &String,
    ) -> Result<String, Box<dyn Error>>
        where
            T: Serialize,
    {
        let resp = client
            .put(&url)
            .header("authorization", format!("sso-key {}:{}", api_key, secret))
            .json(&body)
            .send()
            .await?
            .text()
            .await?;
        Ok(resp)
    }
}

impl DnsProvider for  GoDaddyAPI {


    async fn set_sub_domain(&self, record: &Record) -> () {

        let url = format!(
            "https://api.godaddy.com/v1/domains/{}/records/{}/{}",
            self.domain,
            record.record_type.to_string(),
            record.name
        );

        let body =
            vec![json!({"data": self.ip, "ttl": 600, "type": record.record_type.to_string()})];
        let resp = Self::send_request(&self.client, url, body, &self.api_key, &self.secret).await;
        if let Err(err) = resp {
            panic!(
                "Something went wrong, please create a bug report with this error: {:?}",
                err
            );
        } else if self.debug {
            println!("Ok, Response: {:?}", resp);
        } else {
            println!("The subdomain {} has been registered", record.name);
        }
    }

    async fn remove_sub_domain(&self, record: &Record) -> () {
        todo!("Implement this")
    }
}
