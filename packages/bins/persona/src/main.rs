use clap::Parser;
use persona::{Cli, handle_cli};

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
    // Tracing initialization is now handled in handle_cli based on verbosity
    let cli = Cli::parse();

    handle_cli(cli)?;
    Ok(())
}
