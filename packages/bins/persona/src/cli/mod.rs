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
    Build {
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handle_cli;
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
            Commands::Build { output } => assert!(output.is_none()),
            _ => panic!("Expected Build command"),
        }
    }

    #[test]
    fn test_global_args() {
        // Output is now subcommand specific, so we test build with output separately
        let cli = Cli::parse_from(["persona", "-i", "dir1", "-i", "dir2", "-v", "check"]);
        assert_eq!(
            cli.input,
            vec![PathBuf::from("dir1"), PathBuf::from("dir2")]
        );
        assert_eq!(cli.verbose, 1);
        match cli.command {
            Commands::Check => (),
            _ => panic!("Expected Check command"),
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
    fn test_build_output_arg() {
        let cli = Cli::parse_from(["persona", "build", "-o", "output_dir"]);
        match cli.command {
            Commands::Build { output } => assert_eq!(output, Some(PathBuf::from("output_dir"))),
            _ => panic!("Expected Build command with output"),
        }
    }

    // Integration-style tests to cover handlers
    #[test]
    fn test_handle_cli_check() {
        let cli = Cli {
            input: vec![],
            verbose: 0,
            command: Commands::Check,
        };
        assert!(handle_cli(cli).is_ok());
    }

    #[test]
    fn test_handle_cli_list() {
        let cli = Cli {
            input: vec![],
            verbose: 0,
            command: Commands::List,
        };
        // This might print to stdout, but should return Ok
        assert!(handle_cli(cli).is_ok());
    }

    #[test]
    fn test_handle_cli_build() {
        let cli = Cli {
            input: vec![],
            verbose: 0,
            command: Commands::Build { output: None },
        };
        assert!(handle_cli(cli).is_ok());
    }

    #[test]
    fn test_handle_cli_build_with_output() {
        let cli = Cli {
            input: vec![],
            verbose: 0,
            command: Commands::Build {
                output: Some(PathBuf::from("out")),
            },
        };
        assert!(handle_cli(cli).is_ok());
    }
}
