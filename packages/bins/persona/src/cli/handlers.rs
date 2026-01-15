use persona_core::{list_files, print_files, validate};

use crate::cli::Cli;

#[tracing::instrument(skip(cli))]
pub fn handle_cli(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        crate::cli::Commands::Check => {
            handle_validate_command()?;
        }
        crate::cli::Commands::List { dir } => {
            handle_list_command(&dir)?;
        }
    }
    Ok(())
}

#[tracing::instrument]
fn handle_list_command(dir: &str) -> anyhow::Result<()> {
    let files = list_files(dir)?;
    print_files(&files, std::io::stdout())?;
    Ok(())
}

#[tracing::instrument]
fn handle_validate_command() -> anyhow::Result<()> {
    validate();
    Ok(())
}
