use crate::config::Config;
use crate::discord_webhook::DiscordWebhook;
use crate::webhook_notifier::WebhookNotifierType;
use clap::{Parser, Subcommand};
use cli_program::CLIProgram;
use config::CONFIG_SINGLETON;
use dns_provider::DnsProvider;
use std::path::PathBuf;

mod cli_program;
mod cloudflare_provider;
mod config;
mod discord_webhook;
mod dns_provider;
mod ip_handler;
mod webhook_notifier;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    /// Sets a custom config file. Default file is ~/.config/dns-cli-config.json
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, PartialEq)]
enum Commands {
    /// Prints the current config
    PrintConfig {},

    #[command(subcommand)]
    Cloudflare(DomainCommands),

    /// Creates a new config file at the configured config path or default path.
    Init {},

    /// Commands for interacting with discord webhooks
    #[command(subcommand)]
    Discord(WebhookCommands),
}

#[derive(Subcommand, PartialEq)]
enum DomainCommands {
    /// Checks if IP has changed and if it has changed updates all the DNS entries tied to this server
    Check {
        /// Forces the update of the records even if the IP has not changed
        #[arg(short, long)]
        force: bool,
    },
    /// Creates a new subdomain and ties to to this server by updating the IP it points to to the current IP and adding it to the config file of domains to scrape
    Register {
        /// The prefix will the domain will become this arg.example.org
        prefix: String,
    },

    /// Lists all the subdomains and their record types that are being tracked
    Ls {},

    /// Deletes a subdomain (Not ready yet)
    Rm {
        /// The prefix of the subdomain that should be deleted
        prefix: String,
    },
    /// Imports entries that have the same ip as the current ip
    Import {},
}

#[derive(Subcommand, PartialEq)]
enum WebhookCommands {
    /// Adds a new discord webhook to the config file
    Add {
        /// The url of the discord webhook
        url: String,
    },
    /// Removes a discord webhook from the config file
    Rm {
        /// The url of the discord webhook
        url: String,
    },

    /// Lists all the discord webhooks in the config file
    Ls {},
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let path = Config::get_config_path(cli.config.clone());
    let command = cli.command.expect("No command provided");
    if (command == Commands::Init {}) {
        init(cli.config).await;
        return;
    }
    CONFIG_SINGLETON.lock().await.init(path);
    let config = CONFIG_SINGLETON.lock().await.get();

    match command {
        Commands::PrintConfig {} => {
            println!("{:#?}", config);
        }
        Commands::Init {} => init(cli.config).await,
        Commands::Cloudflare(cmd) => {
            let api = cloudflare_provider::CloudflareProvider::new().await;
            let program = CLIProgram::new(api, cli.debug > 0, config);
            handle_domain_command(cmd, program).await;
        }

        Commands::Discord(cmd) => match cmd {
            WebhookCommands::Add { url } => {
                println!("Adding webhook: {}", url);
                let discord_webhook = DiscordWebhook::new(url.to_string());
                let config_path = Config::get_config_path(cli.config.clone());
                let mut config = Config::get_config(&config_path).unwrap();
                config.add_webhook(WebhookNotifierType::DiscordWebhook(discord_webhook));
                config.write(&config_path);
            }
            WebhookCommands::Rm { url } => {
                println!("Removing webhook: {}", url);
            }
            WebhookCommands::Ls {} => {
                println!("Listing webhooks");
            }
        },
    };
}

async fn handle_domain_command<T: DnsProvider>(cmd: DomainCommands, mut program: CLIProgram<T>) {
    match cmd {
        DomainCommands::Check { force } => program.check_for_new_ip(force.to_owned()).await,
        DomainCommands::Ls {} => program.ls(),
        DomainCommands::Register { prefix } => program.register_sub_domain(prefix).await,
        DomainCommands::Rm { prefix } => program.remove_sub_domain(prefix).await,
        DomainCommands::Import {} => program.import().await,
    }
}

async fn init(path: Option<PathBuf>) {
    let path = Config::get_config_path(path);
    if path.exists() {
        panic!(
            "Config file already exists at {:?}, delete file to continue",
            path
        );
    }

    println!("Creating new config file at path: {:?}", path);
    Config::get_default_config().write(&path);
    println!("Done!");
}
