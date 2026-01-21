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
    #[arg(short, long, global = true, default_value = ".agent")]
    pub input: Vec<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    #[arg(long, global = true, default_value = "5000")]
    pub warn_token_count: u64,

    #[arg(long, global = true, default_value = "10000")]
    pub error_token_count: u64,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    #[command(about = "Validate agent skill definitions")]
    Check {
        #[arg(long, default_value = "AGENTS.md")]
        agents_file: PathBuf,
    },
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
            Commands::Check { agents_file } => assert_eq!(agents_file, PathBuf::from("AGENTS.md")),
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
            Commands::Check { .. } => (),
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

    #[test]
    fn test_token_args_parsing() {
        let cli = Cli::parse_from([
            "persona",
            "--warn-token-count",
            "100",
            "--error-token-count",
            "200",
            "check",
        ]);
        assert_eq!(cli.warn_token_count, 100);
        assert_eq!(cli.error_token_count, 200);
    }

    fn setup_temp_dir(name: &str) -> PathBuf {
        let temp_dir = std::env::temp_dir().join(name);
        if temp_dir.exists() {
            std::fs::remove_dir_all(&temp_dir).unwrap();
        }
        std::fs::create_dir_all(&temp_dir).unwrap();
        temp_dir
    }

    // Integration-style tests to cover handlers
    #[test]
    fn test_handle_cli_check() {
        use persona_core::{collect_entities, xml::generate_xml};

        let temp_dir = setup_temp_dir("persona_test_check");
        let inputs_dir = temp_dir.join("inputs");
        std::fs::create_dir(&inputs_dir).unwrap();
        let inputs = vec![inputs_dir.clone()];

        // Create a dummy skill so we have something to validate
        let skill_dir = inputs_dir.join("skills/test/myskill");
        std::fs::create_dir_all(&skill_dir).unwrap();
        let skill_file = skill_dir.join("SKILL.md");
        let content = "---\nname: myskill\ndescription: Test skill\n---\nBody";
        std::fs::write(&skill_file, content).unwrap();

        // Generate expected AGENTS.md content dynamically
        let entities = collect_entities(&inputs, 5000, 10000).unwrap();
        let xml_content = generate_xml(&entities, &inputs, None).unwrap();

        let agents_file = temp_dir.join("AGENTS.md");
        std::fs::write(&agents_file, xml_content).unwrap();

        let cli = Cli {
            input: inputs,
            verbose: 0,
            warn_token_count: 5000,
            error_token_count: 10000,
            command: Commands::Check { agents_file },
        };

        assert!(handle_cli(cli).is_ok());
        std::fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_handle_cli_check_fails_on_token_limit() {
        use persona_core::{collect_entities, xml::generate_xml};

        let temp_dir = setup_temp_dir("persona_test_check_fail");
        let inputs_dir = temp_dir.join("inputs");
        std::fs::create_dir(&inputs_dir).unwrap();
        let inputs = vec![inputs_dir.clone()];

        // Create a dummy skill
        let skill_dir = inputs_dir.join("skills/test/myskill");
        std::fs::create_dir_all(&skill_dir).unwrap();
        let skill_file = skill_dir.join("SKILL.md");
        // Create large content
        let body = "a".repeat(1000); // 1000 chars = 200 tokens
        let content = format!("---\nname: myskill\ndescription: Test skill\n---\n{}", body);
        std::fs::write(&skill_file, content).unwrap();

        // Generate AGENTS.md
        let entities = collect_entities(&inputs, 5000, 10000).unwrap();
        let xml_content = generate_xml(&entities, &inputs, None).unwrap();
        let agents_file = temp_dir.join("AGENTS.md");
        std::fs::write(&agents_file, xml_content).unwrap();

        let cli = Cli {
            input: inputs,
            verbose: 0,
            warn_token_count: 5000,
            error_token_count: 50, // Limit 50 tokens, content is > 200
            command: Commands::Check { agents_file },
        };

        assert!(handle_cli(cli).is_err());
        std::fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_handle_cli_list() {
        let temp_dir = setup_temp_dir("persona_test_list");

        let cli = Cli {
            input: vec![temp_dir.clone()],
            verbose: 0,
            warn_token_count: 5000,
            error_token_count: 10000,
            command: Commands::List,
        };
        // This might print to stdout, but should return Ok
        assert!(handle_cli(cli).is_ok());
        std::fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_handle_cli_build() {
        let cli = Cli {
            input: vec![],
            verbose: 0,
            warn_token_count: 5000,
            error_token_count: 10000,
            command: Commands::Build { output: None },
        };
        assert!(handle_cli(cli).is_ok());
    }

    #[test]
    fn test_handle_cli_build_with_output() {
        let cli = Cli {
            input: vec![],
            verbose: 0,
            warn_token_count: 5000,
            error_token_count: 10000,
            command: Commands::Build {
                output: Some(PathBuf::from("out")),
            },
        };
        assert!(handle_cli(cli).is_ok());
    }
}
