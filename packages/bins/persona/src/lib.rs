use persona_core::{list_files, print_files, validate};

#[tracing::instrument]
pub fn handle_list_command(dir: &str) -> anyhow::Result<()> {
    let files = list_files(dir)?;
    print_files(&files, std::io::stdout())?;
    Ok(())
}

#[tracing::instrument]
pub fn handle_validate_command() -> anyhow::Result<()> {
    validate();
    Ok(())
}
