use serde::{Deserialize, Serialize};

extern crate lazy_static;
use lazy_static::lazy_static;
use std::{collections::HashMap, path::PathBuf};
use tokio::sync::Mutex;

use crate::webhook_notifier::WebhookNotifierType;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub cloudflare_config: CloudflareConfig,

    #[serde(default)]
    pub webhooks: Vec<WebhookNotifierType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CloudflareConfig {
    pub api_token: String,
    pub domains: HashMap<String, Domain>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Domain {
    pub records: Vec<Record>,
    pub domain: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    pub id: String,
    pub name: String,
    pub record_type: RecordType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl Config {
    pub(crate) fn add_webhook(&mut self, webhook: WebhookNotifierType) {
        self.webhooks.push(webhook);
    }

    pub(crate) fn get_default_config() -> Config {
        Config {
            cloudflare_config: CloudflareConfig {
                api_token: "".to_string(),
                domains: HashMap::new(),
            },
            webhooks: vec![],
        }
    }

    pub fn write(&self, path: &PathBuf) {
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

        let config: Config =
            serde_json::from_str(&config).expect("Failed to parse your config, please validate it");
        Some(config)
    }

    pub(crate) fn get_default_config_path() -> PathBuf {
        home::home_dir()
            .unwrap()
            .join(".config/dns-cli-config.json")
    }

    pub(crate) fn get_config_path(custom_path: Option<PathBuf>) -> PathBuf {
        custom_path.unwrap_or(Config::get_default_config_path())
    }
}

pub struct ConfigSingleton {
    config: Option<Config>,
    path: Option<PathBuf>,
}
impl ConfigSingleton {
    pub fn init(&mut self, path: PathBuf) {
        self.config = Some(Config::get_config(&path).unwrap());
        self.path = Some(path);
    }
    pub fn get(&self) -> Config {
        let config = self.config.clone();
        config.unwrap()
    }
    pub fn save(&mut self, config: Config) {
        let path = self.path.clone().unwrap();
        self.config = Some(config);
        self.config.clone().unwrap().write(&path);
    }
}
lazy_static! {
    pub static ref CONFIG_SINGLETON: Mutex<ConfigSingleton> = Mutex::new(ConfigSingleton {
        config: None,
        path: None
    });
}
