use std::path::PathBuf;

use clap::{Parser, Subcommand};
use serde_json::json;

use crate::config::{Config, Record, RecordType};
use reqwest;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    CheckForNewIP {},
    /// Creates a new subdomain
    RegisterSubDomain {
        /// The prefix will the domain will become this arg.example.org
        prefix: String,
    },
    /// Creates a new config file
    Init {},
}

pub struct CLIProgram {
    cli: Cli,
}

fn get_config(path: PathBuf) -> Option<Config> {
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

fn get_default_config() -> Config {
    Config {
        records: vec![Record {
            name: "sub".to_string(),
            record_type: RecordType::A,
        }],
        domain: "example.com".to_string(),
        api_key: "replace_with_your_api_key".to_string(),
        secret: "replace_with_your_secret".to_string(),
    }
}

fn get_config_path(custom_path: Option<PathBuf>) -> PathBuf {
    custom_path.unwrap_or(PathBuf::from("config.json"))
}
impl CLIProgram {
    pub async fn new() -> CLIProgram {
        let cli = Cli::parse();
        let c = CLIProgram { cli };
        c.start().await;
        c
    }

    async fn start(&self) {
        // You can check for the existence of subcommands, and if found use their
        // matches just as you would the top level cmd
        match &self.cli.command {
            Some(Commands::CheckForNewIP {}) => self.check_for_new_ip().await,
            Some(Commands::RegisterSubDomain { prefix }) => self.register_sub_domain(prefix),
            Some(Commands::Init {}) => self.init(),
            None => {
                println!("Nothing")
            }
        }
    }

    async fn check_for_new_ip(&self) {
        println!("Checking for new ip...");
        let old_ip = Self::get_last_ip();
        let current_ip = Self::get_current_ip().await;
        if old_ip == current_ip {
            println!("IP has not changed, doing nothing");
        } else {
            println!("IP has changed, updating records");
            if let Some(config) = get_config(get_config_path(self.cli.config.clone())) {
                Self::update_records(&current_ip, &config).await;
                Self::save_ip(&current_ip).await;
            }
        }
    }

    fn get_last_ip() -> String {
        let home = home::home_dir().expect("Could not get home dir :(");

        let path = home.join(".last_ip.txt");
        if !path.exists() {
            println!("No last ip file found, probably first run");
            return "".to_string();
        }
        std::fs::read_to_string(path).unwrap()
    }

    async fn get_current_ip() -> String {
        reqwest::get("https://api.ipify.org")
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    }

    async fn save_ip(ip: &String) {
        let home = home::home_dir().expect("Could not get home dir :(");

        let path = home.join(".last_ip.txt");
        std::fs::write(path, ip).unwrap();
    }
    async fn update_records(new_ip: &String, config: &Config) -> () {
        println!("Updating records...");
        let client = reqwest::Client::new();
        for record in &config.records {
            let url = format!(
                "https://api.godaddy.com/v1/domains/{}/records/{}/{}",
                config.domain,
                record.record_type.to_string(),
                record.name
            );
            let body =
                vec![json!({"data": new_ip, "ttl": 600, "type": record.record_type.to_string()})];
            let resp = client
                .put(&url)
                .header(
                    "authorization",
                    format!("sso-key {}:{}", config.api_key, config.secret),
                )
                .json(&body)
                .send()
                .await
                .unwrap()
                .text()
                .await;

            println!("Response: {:?}", resp);
        }
    }

    fn register_sub_domain(&self, prefix: &String) {}

    fn init(&self) {
        let path = get_config_path(self.cli.config.clone());
        if path.exists() {
            println!(
                "Config file already exists at {:?}, delete file to continue",
                path
            );
            return;
        }

        println!("Creating new config file...");
        let config = get_default_config();
        let config_json = serde_json::to_string_pretty(&config).unwrap();
        std::fs::write(path, config_json).unwrap();
        println!("Done!");
    }
}
