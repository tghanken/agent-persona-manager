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

// New types for parsing stages
pub struct ValidatedPath(PathBuf);
pub struct FileContent {
    path: ValidatedPath,
    content: String,
}
pub struct SplitContent {
    path: ValidatedPath,
    frontmatter_str: String,
    body: String,
}

pub trait PersonaParser {
    fn parse(&self, path: &Path) -> Result<ParsedEntity, PersonaError>;
}

pub struct MarkdownParser;

impl PersonaParser for MarkdownParser {
    #[tracing::instrument(skip(self))]
    fn parse(&self, path: &Path) -> Result<ParsedEntity, PersonaError> {
        // Step 1: Validate Path
        let validated_path = ValidatedPath::new(path)?;

        // Step 2: Read Content
        let file_content = FileContent::read(validated_path)?;

        // Step 3: Split Frontmatter
        let split_content = SplitContent::parse(file_content)?;

        // Step 4: Parse & Validate Entity
        ParsedEntity::try_from(split_content)
    }
}

// Implementations for parsing stages

impl ValidatedPath {
    fn new(path: &Path) -> Result<Self, PersonaError> {
        let file_stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| PersonaError::InvalidFilename("".to_string()))?;

        if file_stem.chars().any(|c| c.is_lowercase()) {
            return Err(PersonaError::InvalidFilename(
                path.file_name().unwrap().to_string_lossy().to_string(),
            ));
        }
        Ok(Self(path.to_path_buf()))
    }
}

impl FileContent {
    fn read(path: ValidatedPath) -> Result<Self, PersonaError> {
        let content = std::fs::read_to_string(&path.0)?;
        Ok(Self { path, content })
    }
}

impl SplitContent {
    fn parse(input: FileContent) -> Result<Self, PersonaError> {
        let (frontmatter, body) = extract_frontmatter_and_body(&input.content)?;
        Ok(Self {
            path: input.path,
            frontmatter_str: frontmatter.to_string(),
            body: body.to_string(),
        })
    }
}

impl TryFrom<SplitContent> for ParsedEntity {
    type Error = PersonaError;

    fn try_from(split: SplitContent) -> Result<Self, Self::Error> {
        let frontmatter: Frontmatter = serde_yaml::from_str(&split.frontmatter_str)?;

        // Validate logic
        if !is_valid_name(&frontmatter.name) {
            return Err(PersonaError::InvalidNameFormat(frontmatter.name));
        }

        if frontmatter.description.trim().is_empty() {
            return Err(PersonaError::EmptyDescription);
        }

        let parent_dir_name = split
            .path
            .0
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .ok_or_else(|| {
                PersonaError::ParentDirNotFound(split.path.0.to_string_lossy().to_string())
            })?;

        if frontmatter.name != parent_dir_name {
            return Err(PersonaError::NameMismatch {
                frontmatter_name: frontmatter.name,
                dir_name: parent_dir_name.to_string(),
            });
        }

        if split.body.trim().is_empty() {
            return Err(PersonaError::EmptyBody);
        }

        Ok(ParsedEntity {
            path: split.path.0,
            frontmatter,
            body: split.body,
        })
    }
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

fn is_valid_name(name: &str) -> bool {
    if name.is_empty() || name.len() > 64 {
        return false;
    }
    name.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filename_validation() {
        let valid = PathBuf::from("valid/path/ENTITY.md");
        assert!(ValidatedPath::new(&valid).is_ok());

        let invalid = PathBuf::from("valid/path/entity.md");
        assert!(matches!(
            ValidatedPath::new(&invalid),
            Err(PersonaError::InvalidFilename(_))
        ));
    }

    #[test]
    fn test_frontmatter_extraction() {
        let content = "---\nkey: value\n---\nbody";
        let result = extract_frontmatter_and_body(content);
        assert!(result.is_ok());
        let (fm, body) = result.unwrap();
        assert_eq!(fm.trim(), "key: value");
        assert_eq!(body.trim(), "body");
    }

    #[test]
    fn test_frontmatter_extraction_missing() {
        let content = "body only";
        let result = extract_frontmatter_and_body(content);
        assert!(matches!(result, Err(PersonaError::MissingFrontmatter)));
    }

    #[test]
    fn test_frontmatter_extraction_unclosed() {
        let content = "---\nkey: value\nbody";
        let result = extract_frontmatter_and_body(content);
        assert!(matches!(result, Err(PersonaError::MissingFrontmatter)));
    }

    #[test]
    fn test_name_validation() {
        assert!(is_valid_name("valid-name-123"));
        assert!(!is_valid_name("InvalidName"));
        assert!(!is_valid_name("name_with_underscore"));
        assert!(!is_valid_name(""));
        assert!(!is_valid_name(&"a".repeat(65)));
    }
}
