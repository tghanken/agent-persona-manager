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
    #[error("Name mismatch: Frontmatter name '{frontmatter_name}' does not match parent directory '{dir_name}'")]
    NameMismatch {
        frontmatter_name: String,
        dir_name: String,
    },
    #[error("Invalid name: '{0}'. Must be 1-64 chars, lowercase alphanumeric and hyphens.")]
    InvalidNameFormat(String),
    #[error("Missing or empty description")]
    EmptyDescription,
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
        return Err(PersonaError::InvalidFilename(path.file_name().unwrap().to_string_lossy().to_string()));
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

    // 7. Validate Name matches Parent Directory
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
    name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    fn create_temp_file(dir: &Path, file_name: &str, content: &str) -> PathBuf {
        let file_path = dir.join(file_name);
        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file_path
    }

    struct TestContext {
        root: PathBuf,
    }

    impl TestContext {
        fn new(name: &str) -> Self {
            let root = std::env::temp_dir().join(format!("persona_parser_test_{}_{}", name, std::process::id()));
            if root.exists() {
                fs::remove_dir_all(&root).unwrap();
            }
            fs::create_dir_all(&root).unwrap();
            Self { root }
        }
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            if self.root.exists() {
                let _ = fs::remove_dir_all(&self.root);
            }
        }
    }

    #[test]
    fn test_valid_parsing() {
        let ctx = TestContext::new("valid");
        let entity_dir = ctx.root.join("test-entity");
        fs::create_dir(&entity_dir).unwrap();
        let file_path = create_temp_file(
            &entity_dir,
            "ENTITY.md",
            "---\nname: test-entity\ndescription: Test Description\n---\nBody content"
        );

        let result = parse_file(&file_path);
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.frontmatter.name, "test-entity");
        assert_eq!(entity.frontmatter.description, "Test Description");
        assert_eq!(entity.body, "Body content");
    }

    #[test]
    fn test_invalid_filename_case() {
        let ctx = TestContext::new("filename_case");
        let entity_dir = ctx.root.join("test-entity");
        fs::create_dir(&entity_dir).unwrap();
        let file_path = create_temp_file(
            &entity_dir,
            "entity.md",
            "---\nname: test-entity\ndescription: desc\n---\n"
        );

        let result = parse_file(&file_path);
        assert!(matches!(result, Err(PersonaError::InvalidFilename(_))));
    }

    #[test]
    fn test_name_mismatch() {
        let ctx = TestContext::new("mismatch");
        let entity_dir = ctx.root.join("dir-name");
        fs::create_dir(&entity_dir).unwrap();
        let file_path = create_temp_file(
            &entity_dir,
            "ENTITY.md",
            "---\nname: other-name\ndescription: desc\n---\n"
        );

        let result = parse_file(&file_path);
        match result {
            Err(PersonaError::NameMismatch { frontmatter_name, dir_name }) => {
                assert_eq!(frontmatter_name, "other-name");
                assert_eq!(dir_name, "dir-name");
            }
            _ => panic!("Expected NameMismatch error"),
        }
    }

    #[test]
    fn test_invalid_name_format() {
        let ctx = TestContext::new("name_format");
        let entity_dir = ctx.root.join("InvalidName");
        fs::create_dir(&entity_dir).unwrap();
        let file_path = create_temp_file(
            &entity_dir,
            "ENTITY.md",
            "---\nname: InvalidName\ndescription: desc\n---\n"
        );

        let result = parse_file(&file_path);
        assert!(matches!(result, Err(PersonaError::InvalidNameFormat(_))));
    }

    #[test]
    fn test_missing_frontmatter() {
        let ctx = TestContext::new("missing_fm");
        let entity_dir = ctx.root.join("test-entity");
        fs::create_dir(&entity_dir).unwrap();
        let file_path = create_temp_file(
            &entity_dir,
            "ENTITY.md",
            "Just body content"
        );

        let result = parse_file(&file_path);
        assert!(matches!(result, Err(PersonaError::MissingFrontmatter)));
    }

    #[test]
    fn test_empty_description() {
        let ctx = TestContext::new("empty_desc");
        let entity_dir = ctx.root.join("test-entity");
        fs::create_dir(&entity_dir).unwrap();
        let file_path = create_temp_file(
            &entity_dir,
            "ENTITY.md",
            "---\nname: test-entity\ndescription: \n---\n"
        );

        let result = parse_file(&file_path);
        assert!(matches!(result, Err(PersonaError::EmptyDescription)));
    }

    #[test]
    fn test_nested_triples_in_content() {
        let ctx = TestContext::new("nested");
        let entity_dir = ctx.root.join("test-nested");
        fs::create_dir(&entity_dir).unwrap();
        let content = "---\nname: test-nested\ndescription: desc\n---\nHere is some code:\n```\n---\n```";
        let file_path = create_temp_file(
            &entity_dir,
            "ENTITY.md",
            content
        );

        let result = parse_file(&file_path);
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.frontmatter.name, "test-nested");
        assert_eq!(entity.body.trim(), "Here is some code:\n```\n---\n```");
    }
}
