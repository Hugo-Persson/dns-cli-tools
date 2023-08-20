use cli_program::CLIProgram;

mod cli_program;
mod config;
mod godaddy_api;

#[tokio::main]
async fn main() {
    let _program = CLIProgram::new().await;
}
