use crate::config::{Config, Record, RecordType, CONFIG_SINGLETON};
use crate::dns_provider::DnsProvider;

use crate::ip_handler::{get_current_ip, get_last_ip, save_ip};
use crate::webhook_notifier::WebhookNotifier;
use crate::webhook_notifier::WebhookNotifierType::DiscordWebhook;

pub struct CLIProgram<T>
where
    T: DnsProvider,
{
    debug: bool,
    config: Config,
    api: T,
}

impl<T> CLIProgram<T>
where
    T: DnsProvider,
{
    pub fn new(api: T, debug: bool, config: Config) -> CLIProgram<T> {
        CLIProgram { debug, config, api }
    }

    pub(crate) async fn check_for_new_ip(&self, force: bool) {
        println!("Checking for new ip...");
        let old_ip = get_last_ip(self.debug);
        let current_ip = get_current_ip().await;
        if old_ip.is_none() {
            println!("No previous IP found, saving current IP");
            save_ip(&current_ip).await;
            return;
        }
        if old_ip == current_ip.unwrap && !force {
            println!("IP has not changed, doing nothing");
        } else {
            if force {
                println!("IP has not changed but force flag set, updating records...")
            } else {
                println!(
                    "IP has changed to {} from {}, updating records...",
                    current_ip, old_ip
                );
            }
            println!("Notifying webhooks: {:?}", self.config.webhooks);
            for webhook in &self.config.webhooks {
                match webhook {
                    DiscordWebhook(webhook) => {
                        webhook.change_ip(&old_ip, &current_ip).await;
                    }
                }
            }
            self.update_records(&current_ip).await;
            save_ip(&current_ip).await;
        }
    }

    pub async fn remove_sub_domain(&mut self, domain: String) {
        let domain_chunks = domain.split('.').collect::<Vec<&str>>();
        let zone_name = format!("{}.{}", domain_chunks[1], domain_chunks[2]); // TODO:
                                                                              // Support for more than 3 chunks
        println!("Removing subdomain {} for {}", domain_chunks[0], zone_name);
        let zone = self
            .config
            .cloudflare_config
            .domains
            .iter()
            .find(|e| e.1.domain == zone_name)
            .expect("Domain not found");
        let record_index = zone
            .1
            .records
            .iter()
            .position(|e| e.name == domain)
            .expect("Record not found");
        let record = zone.1.records.get(record_index).unwrap();
        self.api.remove_sub_domain(record, zone.0.to_owned()).await;
        let domains = self.config.cloudflare_config.domains.clone();

        let key = domains.keys().next().unwrap();
        self.config
            .cloudflare_config
            .domains
            .get_mut(key)
            .unwrap()
            .records
            .swap_remove(record_index);
        CONFIG_SINGLETON.lock().await.save(self.config.clone())
    }

    async fn update_records(&self, new_ip: &str) {
        println!("Updating records...");
        self.api.change_ip(new_ip).await;
    }

    pub(crate) async fn register_sub_domain(&mut self, domain: String) {
        let domain_chunks = domain.split('.').collect::<Vec<&str>>();
        let zone_name = format!("{}.{}", domain_chunks[1], domain_chunks[2]); // TODO:
                                                                              // Support for more than 3 chunks
        println!(
            "Registering subdomain {} for {}",
            domain_chunks[0], zone_name
        );
        let zone_id = self
            .config
            .cloudflare_config
            .domains
            .iter()
            .find(|e| e.1.domain == zone_name)
            .expect("Domain not found");
        let id = self
            .api
            .set_sub_domain(
                &Record {
                    id: "".to_string(),
                    name: domain.to_string(),
                    record_type: RecordType::A,
                },
                zone_id.0.to_owned(),
            )
            .await;
        let domains = self.config.cloudflare_config.domains.clone();

        let key = domains.keys().next().unwrap();
        self.config
            .cloudflare_config
            .domains
            .get_mut(key)
            .unwrap()
            .records
            .push(Record {
                id,
                name: domain.to_owned(),
                record_type: RecordType::A,
            });
        CONFIG_SINGLETON.lock().await.save(self.config.clone())
    }

    pub fn ls(&self) {
        for (_, domain) in &self.config.cloudflare_config.domains {
            for record in &domain.records {
                println!(
                    "{}.{} - {}",
                    record.name,
                    domain.domain,
                    record.record_type.to_string()
                );
            }
        }
    }

    pub async fn import(&mut self) {
        self.api.import().await;
    }
}

// Helpers
