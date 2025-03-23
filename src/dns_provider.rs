use crate::config::Record;

pub trait DnsProvider {
    async fn set_sub_domain(&self, record: &Record, zone_id: String) -> String;

    async fn remove_sub_domain(&self, record: &Record, zone_id: String);

    async fn change_ip(&self, ip: &str);
    async fn import(&mut self);

    async fn get_domain_details(&self, prefix: &str) -> Result<DomainDetails, Box<dyn std::error::Error>>;
}

#[derive(Debug)]
pub struct DomainDetails {
    pub name: String,
    pub record_type: String,
    pub content: String,
    pub proxied: bool,
    pub ttl: u32,
    pub modified_on: Option<String>,
}
