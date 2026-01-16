use serde::Deserialize;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PersonaError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("Invalid filename: '{0}'. Must be ALL CAPS (e.g., SKILL.md)")]
    InvalidFilename(String),
    #[error("Missing frontmatter")]
    MissingFrontmatter,
    #[error(
        "Name mismatch: Frontmatter name '{frontmatter_name}' does not match parent directory '{dir_name}'"
    )]
    NameMismatch {
        frontmatter_name: String,
        dir_name: String,
    },
    #[error("Invalid name: '{0}'. Must be 1-64 chars, lowercase alphanumeric and hyphens.")]
    InvalidNameFormat(String),
    #[error("Missing or empty description")]
    EmptyDescription,
    #[error("Missing or empty body content")]
    EmptyBody,
    #[error("Parent directory not found for path: {0}")]
    ParentDirNotFound(String),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Frontmatter {
    pub name: String,
    pub description: String,
    #[serde(flatten)]
    pub other: serde_yaml::Value,
}

#[derive(Debug)]
pub struct ParsedEntity {
    pub path: PathBuf,
    pub frontmatter: Frontmatter,
    pub body: String,
}

#[tracing::instrument]
pub fn parse_file(path: &Path) -> Result<ParsedEntity, PersonaError> {
    // 1. Validate filename is ALL CAPS
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| PersonaError::InvalidFilename("".to_string()))?;

    // Check if filename contains lowercase letters.
    // The requirement is "ALL CAPS filename".
    // We check the stem (without extension).
    if file_stem.chars().any(|c| c.is_lowercase()) {
        return Err(PersonaError::InvalidFilename(
            path.file_name().unwrap().to_string_lossy().to_string(),
        ));
    }

    // 2. Read file
    let content = std::fs::read_to_string(path)?;

    // 3. Extract Frontmatter
    // We expect the file to start with ---
    let trimmed_content = content.trim_start();
    if !trimmed_content.starts_with("---") {
        return Err(PersonaError::MissingFrontmatter);
    }

    // Find the end of the frontmatter.
    // We start searching after the first "---".
    // We look for a line that is exactly "---".

    // Let's use a robust search loop.
    let mut current_idx = 3; // Length of first "---"
    let mut fm_end_idx = 0;
    let mut body_start_idx = 0;
    let mut found = false;

    // Search for the closing separator
    // We iterate over potential positions of "\n---"
    while let Some(idx) = trimmed_content[current_idx..].find("\n---") {
        let absolute_idx = current_idx + idx;
        // Check what comes after "---"
        let after_sep = &trimmed_content[absolute_idx + 4..]; // "\n---" is 4 chars

        // It must be followed by newline or EOF to be a valid separator line (assuming "---" is the whole line)
        if after_sep.is_empty() || after_sep.starts_with('\n') || after_sep.starts_with("\r\n") {
            // Found it
            fm_end_idx = absolute_idx;
            body_start_idx = absolute_idx + 4;
            // consume the following newline if present
            if after_sep.starts_with("\r\n") {
                body_start_idx += 2;
            } else if after_sep.starts_with('\n') {
                body_start_idx += 1;
            }
            found = true;
            break;
        }
        // Not a separator line (e.g. "\n---foo"), continue search
        current_idx = absolute_idx + 1;
    }

    if !found {
        return Err(PersonaError::MissingFrontmatter);
    }

    let frontmatter_part = &trimmed_content[3..fm_end_idx];
    let body_part = &trimmed_content[body_start_idx..];

    // 4. Parse Frontmatter
    let frontmatter: Frontmatter = serde_yaml::from_str(frontmatter_part)?;

    // 5. Validate Name Format
    if !is_valid_name(&frontmatter.name) {
        return Err(PersonaError::InvalidNameFormat(frontmatter.name));
    }

    // 6. Validate Description
    if frontmatter.description.trim().is_empty() {
        return Err(PersonaError::EmptyDescription);
    }

    // 7. Validate Body Content
    if body_part.trim().is_empty() {
        return Err(PersonaError::EmptyBody);
    }

    // 8. Validate Name matches Parent Directory
    let parent_dir_name = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .ok_or_else(|| PersonaError::ParentDirNotFound(path.to_string_lossy().to_string()))?;

    if frontmatter.name != parent_dir_name {
        return Err(PersonaError::NameMismatch {
            frontmatter_name: frontmatter.name,
            dir_name: parent_dir_name.to_string(),
        });
    }

    Ok(ParsedEntity {
        path: path.to_path_buf(),
        frontmatter,
        body: body_part.to_string(),
    })
}

fn is_valid_name(name: &str) -> bool {
    if name.is_empty() || name.len() > 64 {
        return false;
    }
    name.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}
