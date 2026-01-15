use clap::{Parser, Subcommand};
use persona::{handle_list_command, handle_validate_command};

#[derive(Parser)]
#[command(version, name = "persona", about = "Persona CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Check,
    List {
        #[arg(default_value = ".agent")]
        dir: String,
    },
}

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Check => {
            handle_validate_command()?;
        }
        Commands::List { dir } => {
            handle_list_command(dir)?;
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }

    #[test]
    fn test_check_command_parsing() {
        let cli = Cli::parse_from(["persona", "check"]);
        if let Commands::Check = cli.command {
            // Worked
        } else {
            panic!("Failed to parse check command");
        }
    }
}
