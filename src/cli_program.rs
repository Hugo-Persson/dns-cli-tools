use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{
    config::{Config, Record, RecordType},
    godaddy_api::GoDaddyAPI,
};
use reqwest;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    /// Sets a custom config file. Default file is ~/.config/godaddy-config.json
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
    /// Checks if IP has changed and if it has changed updates all the DNS entries tied to this server
    CheckForNewIP {
        /// Forces the update of the records even if the IP has not changed
        #[arg(short, long)]
        force: bool,
    },
    /// Creates a new subdomain
    RegisterSubDomain {
        /// The prefix will the domain will become this arg.example.org
        prefix: String,
    },
    /// Creates a new config file
    Init {},

    /// Lists all the subdomains and their record types
    Ls {},
}

pub struct CLIProgram {
    cli: Cli,
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
            Some(Commands::CheckForNewIP { force }) => {
                self.check_for_new_ip(force.to_owned()).await
            }
            Some(Commands::Ls {}) => self.ls(),
            Some(Commands::RegisterSubDomain { prefix }) => self.register_sub_domain(prefix).await,
            Some(Commands::Init {}) => self.init(),
            None => {
                println!("Nothing")
            }
        }
    }

    async fn check_for_new_ip(&self, force: bool) -> () {
        println!("Checking for new ip...");
        let old_ip = get_last_ip();
        let current_ip = get_current_ip().await;
        if old_ip == current_ip && !force {
            println!("IP has not changed, doing nothing");
        } else {
            if force {
                println!("Force flag set, updating records")
            } else {
                println!("IP has changed, updating records");
            }
            if let Some(config) = get_config(&get_config_path(self.cli.config.clone())) {
                Self::update_records(&current_ip, &config).await;
                save_ip(&current_ip).await;
            }
        }
    }

    async fn update_records(new_ip: &String, config: &Config) -> () {
        println!("Updating records...");
        let api = GoDaddyAPI::new(
            config.api_key.clone(),
            config.secret.clone(),
            config.domain.clone(),
            new_ip.to_string(),
        );

        for record in &config.records {
            println!("Updating record: {:?}", record);
            api.put_sub_domain(record).await;
        }
    }

    async fn register_sub_domain(&self, prefix: &String) -> () {
        let path = get_config_path(self.cli.config.clone());
        if let Some(mut config) = get_config(&path) {
            println!("Registering subdomain {}.{} ...", prefix, config.domain);
            let api = GoDaddyAPI::new(
                config.api_key.clone(),
                config.secret.clone(),
                config.domain.clone(),
                get_current_ip().await,
            );
            api.put_sub_domain(&Record {
                name: prefix.to_string(),
                record_type: RecordType::A,
            })
            .await;

            config.records.push(Record {
                name: prefix.to_owned(),
                record_type: RecordType::A,
            });
            write_config(&path, &config);
        }
    }

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
        write_config(&path, &config);
        println!("Done!");
    }

    fn ls(&self) {
        let path = get_config_path(self.cli.config.clone());
        if let Some(config) = get_config(&path) {
            for record in &config.records {
                println!(
                    "{}.{} - {}",
                    record.name,
                    config.domain,
                    record.record_type.to_string()
                );
            }
        }
    }
}

// Helpers

fn write_config(path: &PathBuf, config: &Config) {
    let config_json = serde_json::to_string_pretty(&config).unwrap();
    std::fs::write(path, config_json).unwrap();
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

fn get_config(path: &PathBuf) -> Option<Config> {
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
    custom_path.unwrap_or(
        home::home_dir()
            .unwrap()
            .join(".config/godaddy-config.json"),
    )
}
