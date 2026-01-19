use persona_core::{collect_entities, print_hierarchy};
use std::path::PathBuf;
use tracing::Level;

use crate::cli::{Cli, Commands};

#[tracing::instrument(skip(cli))]
pub fn handle_cli(cli: Cli) -> anyhow::Result<()> {
    // Initialize tracing based on verbosity
    let log_level = match cli.verbose {
        0 => Level::WARN,
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE,
    };

    let _ = tracing_subscriber::fmt()
        .with_max_level(log_level)
        .try_init();

    match cli.command {
        Commands::Check => {
            handle_check_command(&cli.input)?;
        }
        Commands::List => {
            handle_list_command(&cli.input)?;
        }
        Commands::Build { output } => {
            handle_build_command(&cli.input, output.as_deref())?;
        }
    }
    Ok(())
}

#[tracing::instrument]
fn handle_list_command(inputs: &[PathBuf]) -> anyhow::Result<()> {
    let entities = collect_entities(inputs)?;
    print_hierarchy(&entities, inputs, std::io::stdout())?;
    Ok(())
}

#[tracing::instrument]
fn handle_check_command(inputs: &[PathBuf]) -> anyhow::Result<()> {
    collect_entities(inputs)?;
    Ok(())
}

#[tracing::instrument]
fn handle_build_command(
    inputs: &[PathBuf],
    output: Option<&std::path::Path>,
) -> anyhow::Result<()> {
    // Stub implementation
    // In a real implementation, this would read inputs and generate AGENTS.md
    println!("Building agent knowledge summary...");
    if let Some(out) = output {
        println!("Output directory: {}", out.display());
    }
    if !inputs.is_empty() {
        println!("Inputs: {:?}", inputs);
    }
    Ok(())
}