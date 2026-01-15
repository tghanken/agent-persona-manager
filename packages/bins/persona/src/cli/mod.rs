pub mod handlers;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    version,
    name = "persona",
    about = "Manage your custom agent instructions",
    long_about = "Persona is a CLI tool for managing agent instructions. It allows you to list available custom directions and validate their definitions to ensure they are correctly configured for use by agents."
)]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub input: Vec<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    #[arg(short, long, global = true)]
    pub output: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
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
            Commands::Check => (),
            _ => panic!("Expected Check command"),
        }
    }

    #[test]
    fn test_list_command_parsing() {
        let cli = Cli::parse_from(["persona", "list"]);
        match cli.command {
            Commands::List => (),
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_build_command_parsing() {
        let cli = Cli::parse_from(["persona", "build"]);
        match cli.command {
            Commands::Build => (),
            _ => panic!("Expected Build command"),
        }
    }

    #[test]
    fn test_global_args() {
        let cli = Cli::parse_from(["persona", "-i", "dir1", "-i", "dir2", "-v", "-o", "out", "build"]);
        assert_eq!(cli.input, vec![PathBuf::from("dir1"), PathBuf::from("dir2")]);
        assert_eq!(cli.verbose, 1);
        assert_eq!(cli.output, Some("out".to_string()));
        match cli.command {
            Commands::Build => (),
            _ => panic!("Expected Build command"),
        }
    }

    #[test]
    fn test_input_arg() {
        let cli = Cli::parse_from(["persona", "-i", "test_dir", "list"]);
        assert_eq!(cli.input, vec![PathBuf::from("test_dir")]);
    }

    #[test]
    fn test_verbose_arg() {
        let cli = Cli::parse_from(["persona", "-vv", "check"]);
        assert_eq!(cli.verbose, 2);
    }

    #[test]
    fn test_output_arg() {
        let cli = Cli::parse_from(["persona", "-o", "output_dir", "build"]);
        assert_eq!(cli.output, Some("output_dir".to_string()));
    }
}
