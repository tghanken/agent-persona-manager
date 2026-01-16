use persona_parser::{
    MarkdownParser, PersonaError, PersonaParser, ValidatedPath, extract_frontmatter_and_body,
    is_valid_name,
};
use std::path::PathBuf;

// Unit tests for individual components

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
    assert!(!is_valid_name("name_with_underscore")); // alphanumeric and hyphens only
    assert!(!is_valid_name(""));
    assert!(!is_valid_name(&"a".repeat(65)));
}

// Integration-style test for full flow using the Parser trait
// We'll use the inlined content approach to be Nix-safe, but now we test the trait directly.
// We need to write to disk because the trait takes a path and reads it.
// To avoid writing to disk in unit tests (as requested), we mostly rely on the component tests above.
// However, the `parse` method does Step 1 -> 4. We can test one happy path integration.

#[test]
fn test_full_parse_flow() {
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();
    let dir_path = root.join("test-entity");
    fs::create_dir_all(&dir_path).unwrap();
    let file_path = dir_path.join("ENTITY.md");
    let content = r#"---
name: test-entity
description: Test Description
---
Body content"#;
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();

    let parser = MarkdownParser;
    let result = parser.parse(&file_path);
    assert!(result.is_ok());
    let entity = result.unwrap();
    assert_eq!(entity.frontmatter.name, "test-entity");
}
