use persona_core::{list_files, print_files, validate};

use crate::cli::{Cli, Commands};

#[tracing::instrument(skip(cli))]
pub fn handle_cli(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Some(Commands::Check) => {
            handle_check_command(&cli.input)?;
        }
        Some(Commands::List) => {
            handle_list_command(&cli.input)?;
        }
        Some(Commands::Build) => {
            handle_build_command(&cli.input, cli.output.as_deref())?;
        }
        None => {
            // Default command is Build
            handle_build_command(&cli.input, cli.output.as_deref())?;
        }
    }
    Ok(())
}

#[tracing::instrument]
fn handle_list_command(inputs: &[String]) -> anyhow::Result<()> {
    let mut all_files = Vec::new();

    // If no inputs are provided, default to ".agent"
    let inputs_to_use = if inputs.is_empty() {
        vec![".agent".to_string()]
    } else {
        inputs.to_vec()
    };

    for dir in &inputs_to_use {
        match list_files(dir) {
            Ok(files) => all_files.extend(files),
            Err(e) => tracing::warn!("Error listing files in {}: {}", dir, e),
        }
    }
    print_files(&all_files, std::io::stdout())?;
    Ok(())
}

#[tracing::instrument]
fn handle_check_command(_inputs: &[String]) -> anyhow::Result<()> {
    // Current validate implementation in core doesn't take arguments.
    // It's a stub. We just call it.
    validate();
    Ok(())
}

#[tracing::instrument]
fn handle_build_command(inputs: &[String], output: Option<&str>) -> anyhow::Result<()> {
    // Stub implementation
    // In a real implementation, this would read inputs and generate AGENTS.md
    println!("Building agent knowledge summary...");
    if let Some(out) = output {
        println!("Output directory: {}", out);
    }
    if !inputs.is_empty() {
        println!("Inputs: {:?}", inputs);
    }
    Ok(())
}
