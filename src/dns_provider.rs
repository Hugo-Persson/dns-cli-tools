use crate::config::Record;

pub trait DnsProvider {
    async fn set_sub_domain(&self, record: &Record, zone_id: String) -> String;

    async fn remove_sub_domain(&self, record: &Record, zone_id: String);

    async fn change_ip(&self, ip: &str);
    async fn import(&mut self);
}
