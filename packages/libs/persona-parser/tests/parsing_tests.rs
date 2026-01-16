use persona_parser::{parse_file, PersonaError};
use std::path::PathBuf;

fn fixture_path(path: &str) -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/fixtures");
    d.push(path);
    d
}

#[test]
fn test_valid_parsing() {
    let file_path = fixture_path("valid/test-entity/ENTITY.md");
    let result = parse_file(&file_path);
    assert!(result.is_ok(), "Failed to parse valid entity: {:?}", result.err());
    let entity = result.unwrap();
    assert_eq!(entity.frontmatter.name, "test-entity");
    assert_eq!(entity.frontmatter.description, "Test Description");
    assert_eq!(entity.body.trim(), "Body content");
}

#[test]
fn test_invalid_filename_case() {
    let file_path = fixture_path("invalid/filename_case/test-entity/entity.md");
    let result = parse_file(&file_path);
    assert!(matches!(result, Err(PersonaError::InvalidFilename(_))));
}

#[test]
fn test_name_mismatch() {
    let file_path = fixture_path("invalid/mismatch/dir-name/ENTITY.md");
    let result = parse_file(&file_path);
    match result {
        Err(PersonaError::NameMismatch { frontmatter_name, dir_name }) => {
            assert_eq!(frontmatter_name, "other-name");
            assert_eq!(dir_name, "dir-name");
        }
        _ => panic!("Expected NameMismatch error, got {:?}", result),
    }
}

#[test]
fn test_invalid_name_format() {
    let file_path = fixture_path("invalid/name_format/InvalidName/ENTITY.md");
    let result = parse_file(&file_path);
    assert!(matches!(result, Err(PersonaError::InvalidNameFormat(_))));
}

#[test]
fn test_missing_frontmatter() {
    let file_path = fixture_path("invalid/missing_fm/test-entity/ENTITY.md");
    let result = parse_file(&file_path);
    assert!(matches!(result, Err(PersonaError::MissingFrontmatter)));
}

#[test]
fn test_empty_description() {
    let file_path = fixture_path("invalid/empty_desc/test-entity/ENTITY.md");
    let result = parse_file(&file_path);
    assert!(matches!(result, Err(PersonaError::EmptyDescription)));
}

#[test]
fn test_nested_triples_in_content() {
    let file_path = fixture_path("valid/nested/test-nested/ENTITY.md");
    let result = parse_file(&file_path);
    assert!(result.is_ok());
    let entity = result.unwrap();
    assert_eq!(entity.frontmatter.name, "test-nested");
    assert_eq!(entity.body.trim(), "Here is some code:\n```\n---\n```");
}
