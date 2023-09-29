use std::path::PathBuf;


use crate::dns_provider::DnsProvider;
use crate::{config::{Config, Record, RecordType}, config, godaddy_api::GoDaddyAPI};
use reqwest;
use crate::ip_handler::{get_current_ip, get_last_ip, save_ip};
use crate::webhook_notifier::WebhookNotifier;
use crate::webhook_notifier::WebhookNotifierType::DiscordWebhook;

pub struct CLIProgram<T> where T: DnsProvider{
    debug: bool,
    config: Config,
    api: T,
    custom_path: Option<PathBuf>,
}

impl<T> CLIProgram<T> where T: DnsProvider {
    pub fn new(api: T, debug: bool, custom_path: Option<PathBuf>, config: Config) -> CLIProgram<T> {
         CLIProgram { debug, config, api, custom_path}
    }

    pub(crate) async fn check_for_new_ip(&self, force: bool) -> () {
        println!("Checking for new ip...");
        let old_ip = get_last_ip(self.debug);
        let current_ip = get_current_ip().await;
        if old_ip == current_ip && !force {
            println!("IP has not changed, doing nothing");
        } else {
            if force {
                println!("IP has not changed but force flag set, updating records...")
            } else {
                println!("IP has changed, updating records...");

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

    async fn update_records(&self, new_ip: &String, ) -> () {
        println!("Updating records...");
        let api = self.api.change_ip(&new_ip);

        for record in &self.config.records {
            println!("Updating record: {:?}", record);
            api.set_sub_domain(record).await;
        }
    }

    pub(crate) async fn register_sub_domain(&mut self, prefix: &String) -> () {
            println!("Registering subdomain {}.{} ...", prefix, &self.config.domain);
            &self.api.set_sub_domain(&Record {
                name: prefix.to_string(),
                record_type: RecordType::A,
            })
            .await;

            self.config.records.push(Record {
                name: prefix.to_owned(),
                record_type: RecordType::A,
            });

        let path = Config::get_config_path(self.custom_path.clone());
        self.config.write(&path);

        }




    pub fn ls(&self) {
        for record in &self.config.records {
            println!(
                "{}.{} - {}",
                record.name,
                &self.config.domain,

                record.record_type.to_string()
            );
        }
    }
}

// Helpers
