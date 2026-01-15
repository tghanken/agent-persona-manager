use clap::{Parser, Subcommand};
use persona_core::{hello, list_files, validate};

#[derive(Parser)]
#[command(version, name = "persona", about = "Persona CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Check,
    List {
        #[arg(default_value = ".agent")]
        dir: String,
    },
}

fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Check) => {
            validate();
        }
        Some(Commands::List { dir }) => {
            if let Err(_e) = list_files(dir, std::io::stdout()) {
                // tracing::error! is already called inside list_files for directory not found,
                // but we might want to log other IO errors here or just exit.
                // For now, list_files handles the specific error case with logging.
                // If we get an error here, it's propagated.
                // We can use the error to set exit code if needed.
                std::process::exit(1);
            }
        }
        None => {
            hello();
        }
    }
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
        if let Some(Commands::Check) = cli.command {
            // Worked
        } else {
            panic!("Failed to parse check command");
        }
    }
}
