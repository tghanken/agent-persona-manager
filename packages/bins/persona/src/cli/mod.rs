pub mod handlers;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    version,
    name = "persona",
    about = "Manage your custom agent instructions",
    long_about = "Persona is a CLI tool for managing agent instructions. It allows you to list available custom directions and validate their definitions to ensure they are correctly configured for use by agents."
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Validate agent skill definitions")]
    Check,
    #[command(about = "List available agent skills")]
    List {
        #[arg(default_value = ".agent")]
        dir: String,
    },
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
        let _cli = Cli::parse_from(["persona", "check"]);
    }

    #[test]
    fn test_list_command_parsing() {
        let _cli = Cli::parse_from(["persona", "list"]);
    }
}
