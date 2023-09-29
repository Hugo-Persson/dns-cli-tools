#![feature(async_fn_in_trait)]

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use cli_program::CLIProgram;
use crate::config::Config;
use crate::discord_webhook::DiscordWebhook;
use crate::godaddy_api::GoDaddyAPI;
use crate::ip_handler::get_current_ip;
use crate::webhook_notifier::WebhookNotifierType;

mod cli_program;
mod config;
mod godaddy_api;
mod dns_provider;
mod discord_webhook;
mod webhook_notifier;
mod ip_handler;

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

#[derive(Subcommand, PartialEq)]
enum Commands {

    /// Commands for interacting with the godaddy api
    #[command(subcommand)]
    Godaddy(DomainCommands),

    /// Creates a new config file at the configured config path or default path.
    Init {},

    /// Commands for interacting with discord webhooks
    #[command(subcommand)]
    Discord(WebhookCommands),

}

#[derive(Subcommand, PartialEq)]
enum DomainCommands{

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
}

#[derive(Subcommand, PartialEq)]
enum WebhookCommands{
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
    let config = Config::get_config(&path).unwrap();

    match cli.command {
        Commands::Init {} => init(cli.config).await,
        Commands::Godaddy(cmd) => {
            let api = GoDaddyAPI::new(
                config.api_key.clone(),
                config.secret.clone(),
                config.domain.clone(),
                get_current_ip().await,
                cli.debug > 0,
            );
            let program = CLIProgram::new(api, cli.debug>0, cli.config.clone(), config);
            handle_domain_command(cmd, program).await;
        }
        Some(Commands::Discord(cmd)) => {
            match cmd {
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
            }
        }
        None => {
            println!("Nothing")
        }
    }
}

async fn handle_domain_command<T>(cmd: DomainCommands, mut program: CLIProgram<T>) -> () {
    match cmd{

        DomainCommands::Check { force } => program.check_for_new_ip(force.to_owned()).await,
        DomainCommands::Ls {} => program.ls(),
        DomainCommands::Register { prefix } => program.register_sub_domain(&prefix).await,
        DomainCommands::Rm { prefix: _ } => println!("Not implemented yet"),
    }
}

async fn init(path: Option<PathBuf>){
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
