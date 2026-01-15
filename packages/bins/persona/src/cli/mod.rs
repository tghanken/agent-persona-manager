pub mod handlers;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    version,
    name = "persona",
    about = "Manage your custom agent instructions",
    long_about = "Persona is a CLI tool for managing agent instructions. It allows you to list available custom directions and validate their definitions to ensure they are correctly configured for use by agents."
)]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub input: Vec<String>,

    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    #[arg(short, long, global = true)]
    pub output: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    #[command(about = "Validate agent skill definitions")]
    Check,
    #[command(about = "List available agent skills")]
    List,
    #[command(about = "Build the agent knowledge summary")]
    Build,
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
        match cli.command {
            Some(Commands::Check) => (),
            _ => panic!("Expected Check command"),
        }
    }

    #[test]
    fn test_list_command_parsing() {
        let cli = Cli::parse_from(["persona", "list"]);
        match cli.command {
            Some(Commands::List) => (),
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_build_command_parsing() {
        let cli = Cli::parse_from(["persona", "build"]);
        match cli.command {
            Some(Commands::Build) => (),
            _ => panic!("Expected Build command"),
        }
    }

    #[test]
    fn test_default_command_parsing() {
        let cli = Cli::parse_from(["persona"]);
        assert!(cli.command.is_none());
    }

    #[test]
    fn test_global_args() {
        let cli = Cli::parse_from(["persona", "-i", "dir1", "-i", "dir2", "-v", "-o", "out"]);
        assert_eq!(cli.input, vec!["dir1", "dir2"]);
        assert_eq!(cli.verbose, 1);
        assert_eq!(cli.output, Some("out".to_string()));
        assert!(cli.command.is_none());
    }
}
