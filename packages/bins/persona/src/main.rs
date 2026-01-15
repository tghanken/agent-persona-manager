use clap::Parser;
use persona::{Cli, handle_cli};

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    handle_cli(cli)?;
    Ok(())
}
