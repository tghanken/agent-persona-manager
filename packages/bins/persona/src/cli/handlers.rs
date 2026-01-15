use persona_core::{list_files, print_files, validate};
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
    let mut all_files = Vec::new();

    // If no inputs are provided, default to ".agent"
    let inputs_to_use = if inputs.is_empty() {
        vec![PathBuf::from(".agent")]
    } else {
        inputs.to_vec()
    };

    for dir in &inputs_to_use {
        let dir_str = dir.to_string_lossy();
        match list_files(&dir_str) {
            Ok(files) => all_files.extend(files),
            Err(e) => tracing::warn!("Error listing files in {}: {}", dir_str, e),
        }
    }
    print_files(&all_files, std::io::stdout())?;
    Ok(())
}

#[tracing::instrument]
fn handle_check_command(_inputs: &[PathBuf]) -> anyhow::Result<()> {
    // Current validate implementation in core doesn't take arguments.
    // It's a stub. We just call it.
    validate();
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
