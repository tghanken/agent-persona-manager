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

pub trait PersonaParser {
    fn parse(&self, path: &Path) -> Result<ParsedEntity, PersonaError>;
}

pub struct MarkdownParser;

impl PersonaParser for MarkdownParser {
    #[tracing::instrument(skip(self))]
    fn parse(&self, path: &Path) -> Result<ParsedEntity, PersonaError> {
        validate_filename(path)?;

        // 2. Read file
        // We read the whole file because YAML frontmatter parsing and subsequent body extraction
        // are most robustly handled when we have the full context, especially for finding the delimiter.
        // Stream processing is possible but adds significant complexity for delimiter search.
        let content = std::fs::read_to_string(path)?;

        let (frontmatter_str, body) = extract_frontmatter_and_body(&content)?;

        // 4. Parse Frontmatter
        let frontmatter: Frontmatter = serde_yaml::from_str(frontmatter_str)?;

        validate_frontmatter(&frontmatter, path)?;
        validate_body(body)?;

        Ok(ParsedEntity {
            path: path.to_path_buf(),
            frontmatter,
            body: body.to_string(),
        })
    }
}

// Re-export a convenience function for backward compatibility or ease of use
pub fn parse_file(path: &Path) -> Result<ParsedEntity, PersonaError> {
    MarkdownParser.parse(path)
}

fn validate_filename(path: &Path) -> Result<(), PersonaError> {
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| PersonaError::InvalidFilename("".to_string()))?;

    if file_stem.chars().any(|c| c.is_lowercase()) {
        return Err(PersonaError::InvalidFilename(
            path.file_name().unwrap().to_string_lossy().to_string(),
        ));
    }
    Ok(())
}

fn extract_frontmatter_and_body(content: &str) -> Result<(&str, &str), PersonaError> {
    let trimmed_content = content.trim_start();
    if !trimmed_content.starts_with("---") {
        return Err(PersonaError::MissingFrontmatter);
    }

    let mut current_idx = 3; // Length of first "---"
    let mut fm_end_idx = 0;
    let mut body_start_idx = 0;
    let mut found = false;

    while let Some(idx) = trimmed_content[current_idx..].find("\n---") {
        let absolute_idx = current_idx + idx;
        let after_sep = &trimmed_content[absolute_idx + 4..];

        if after_sep.is_empty() || after_sep.starts_with('\n') || after_sep.starts_with("\r\n") {
            fm_end_idx = absolute_idx;
            body_start_idx = absolute_idx + 4;
            if after_sep.starts_with("\r\n") {
                body_start_idx += 2;
            } else if after_sep.starts_with('\n') {
                body_start_idx += 1;
            }
            found = true;
            break;
        }
        current_idx = absolute_idx + 1;
    }

    if !found {
        return Err(PersonaError::MissingFrontmatter);
    }

    Ok((
        &trimmed_content[3..fm_end_idx],
        &trimmed_content[body_start_idx..],
    ))
}

fn validate_frontmatter(frontmatter: &Frontmatter, path: &Path) -> Result<(), PersonaError> {
    if !is_valid_name(&frontmatter.name) {
        return Err(PersonaError::InvalidNameFormat(frontmatter.name.clone()));
    }

    if frontmatter.description.trim().is_empty() {
        return Err(PersonaError::EmptyDescription);
    }

    let parent_dir_name = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .ok_or_else(|| PersonaError::ParentDirNotFound(path.to_string_lossy().to_string()))?;

    if frontmatter.name != parent_dir_name {
        return Err(PersonaError::NameMismatch {
            frontmatter_name: frontmatter.name.clone(),
            dir_name: parent_dir_name.to_string(),
        });
    }
    Ok(())
}

fn validate_body(body: &str) -> Result<(), PersonaError> {
    if body.trim().is_empty() {
        return Err(PersonaError::EmptyBody);
    }
    Ok(())
}

fn is_valid_name(name: &str) -> bool {
    if name.is_empty() || name.len() > 64 {
        return false;
    }
    name.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}
