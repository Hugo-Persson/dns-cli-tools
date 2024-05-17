use crate::config::Record;

pub trait DnsProvider {
    async fn set_sub_domain(&self, record: &Record) -> String;

    async fn remove_sub_domain(&self, record: &Record);

    fn change_ip(&self, ip: &String) -> Self;
    async fn import(&mut self);
}
