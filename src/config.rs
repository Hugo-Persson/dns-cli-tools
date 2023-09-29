use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use crate::webhook_notifier::{WebhookNotifierType};

#[derive(Serialize, Deserialize)]
pub struct Config {

    pub records: Vec<Record>,
    pub domain: String,
    pub api_key: String,
    pub secret: String,

    #[serde(default)]
    pub webhooks: Vec<WebhookNotifierType>,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub name: String,
    pub record_type: RecordType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RecordType {
    A,
    AAA,
}

impl RecordType {
    pub fn to_string(&self) -> String {
        match self {
            RecordType::A => "A".to_string(),
            RecordType::AAA => "AAAA".to_string(),
        }
    }
}

impl Config{
    pub(crate) fn add_webhook(&mut self, webhook: WebhookNotifierType) -> () {
        self.webhooks.push(webhook);
    }

    pub(crate) fn get_default_config() -> Config{

        Config {
            records: vec![Record {
                name: "sub".to_string(),
                record_type: RecordType::A,
            }],
            domain: "example.com".to_string(),
            api_key: "replace_with_your_api_key".to_string(),
            secret: "replace_with_your_secret".to_string(),
            webhooks: vec![],
        }
    }

    pub fn write(&self, path: &PathBuf){
        let config_json = serde_json::to_string_pretty(&self).unwrap();
        std::fs::write(path, config_json).unwrap();
    }

    pub(crate) fn get_config(path: &PathBuf) -> Option<Config> {
        if !path.exists() {
            println!(
                "No config file found for path: {:?}, maybe run the `init` command first?",
                path
            );
            return None;
        }
        let config = std::fs::read_to_string(path).unwrap();
        let config: Config = serde_json::from_str(&config).unwrap();
        Some(config)
    }

    pub(crate) fn get_config_path(custom_path: Option<PathBuf>) -> PathBuf {
        custom_path.unwrap_or(
            home::home_dir()
                .unwrap()
                .join(".config/godaddy-config.json"),
        )
    }
}