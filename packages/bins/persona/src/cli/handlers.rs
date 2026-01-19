use persona_core::{collect_entities, print_hierarchy, xml::generate_xml};
use std::fs;
use std::path::{Path, PathBuf};
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
    let entities = collect_entities(inputs)?;
    let xml_content = generate_xml(&entities, inputs)?;

    fs::write("AGENTS.md", xml_content)?;
    println!("Generated AGENTS.md");

    if let Some(out_dir) = output {
        fs::create_dir_all(out_dir)?;

        for entity in entities {
            let path = &entity.path;
            // Determine relative path
            let mut relative_path = None;
            for input in inputs {
                if let Ok(rel) = path.strip_prefix(input) {
                    relative_path = Some(rel);
                    break;
                }
            }

            if let Some(rel) = relative_path {
                let parent_rel = rel.parent().unwrap_or_else(|| Path::new("."));
                let dest_dir = out_dir.join(parent_rel);

                let src_dir = path
                    .parent()
                    .ok_or_else(|| anyhow::anyhow!("Entity has no parent dir"))?;

                copy_dir_recursive(src_dir, &dest_dir)?;
            }
        }
        println!("Generated output in {}", out_dir.display());
    }
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}
