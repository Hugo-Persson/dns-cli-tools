use crate::config::Record;

pub trait DnsProvider{
    async fn set_sub_domain(&self, record: &Record) -> ();

    async fn remove_sub_domain(&self, record: &Record) -> ();
}