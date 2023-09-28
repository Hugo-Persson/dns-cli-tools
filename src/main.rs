use cli_program::CLIProgram;

mod cli_program;
mod config;
mod godaddy_api;
mod dns_provider;

#[tokio::main]
async fn main() {
    let _program = CLIProgram::new().await;
}
