use cli_program::{CLIProgram, Cli};

mod cli_program;
mod config;

#[tokio::main]
async fn main() {
    let _program = CLIProgram::new().await;
}
