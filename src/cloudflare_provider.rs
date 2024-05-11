use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use cloudflare::endpoints::dns::{DnsContent, DnsRecord};
use cloudflare::endpoints::{dns, zone};
use cloudflare::framework::async_api::Client;
use cloudflare::framework::auth::Credentials;
use cloudflare::framework::{Environment, HttpApiClientConfig};

use crate::dns_provider::DnsProvider;

pub struct CloudflareProvider {
    client: Client,
}
impl CloudflareProvider {
    pub fn new(api_token: String) -> Self {
        let credentials: Credentials = Credentials::UserAuthToken { token: api_token };
        println!("{:?}", credentials);
        let client = Client::new(
            credentials,
            HttpApiClientConfig::default(),
            Environment::Production,
        )
        .expect("Failed to create client");

        Self { client }
    }
    async fn list_zones(&self) -> Vec<zone::Zone> {
        let response = self
            .client
            .request(&zone::ListZones {
                params: zone::ListZonesParams {
                    ..Default::default()
                },
            })
            .await;
        println!("{:?}", response);
        return vec![];
    }
}
impl DnsProvider for CloudflareProvider {
    async fn set_sub_domain(&self, record: &crate::config::Record) {
        let zones = self.list_zones().await;
        println!("{:?}", zones);
    }

    async fn remove_sub_domain(&self, record: &crate::config::Record) {
        todo!()
    }

    fn change_ip(&self, ip: &String) -> Self {
        todo!()
    }
}
