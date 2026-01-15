use clap::{Parser, Subcommand};
use persona_core::{hello, list_files, print_files, validate};

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

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Check) => {
            validate();
        }
        Some(Commands::List { dir }) => {
            let files = list_files(dir)?;
            print_files(&files, std::io::stdout())?;
        }
        None => {
            hello();
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
        if let Some(Commands::Check) = cli.command {
            // Worked
        } else {
            panic!("Failed to parse check command");
        }
    }
}
