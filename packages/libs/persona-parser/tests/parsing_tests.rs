use persona_parser::{PersonaError, parse_file};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;

// Helper to create a file from content
fn create_fixture(dir_structure: &str, file_name: &str, content: &str) -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();
    let dir_path = root.join(dir_structure);
    fs::create_dir_all(&dir_path).unwrap();
    let file_path = dir_path.join(file_name);
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    // Return temp_dir to keep it alive
    (temp_dir, file_path)
}

#[test]
fn test_valid_parsing() {
    let content = r#"---
name: test-entity
description: Test Description
---
Body content"#;
    let (_tmp, path) = create_fixture("test-entity", "ENTITY.md", content);

    let result = parse_file(&path);
    assert!(
        result.is_ok(),
        "Failed to parse valid entity: {:?}",
        result.err()
    );
    let entity = result.unwrap();
    assert_eq!(entity.frontmatter.name, "test-entity");
    assert_eq!(entity.frontmatter.description, "Test Description");
    assert_eq!(entity.body.trim(), "Body content");
    assert!(
        entity
            .frontmatter
            .other
            .as_mapping()
            .is_none_or(|m| m.is_empty()),
        "Frontmatter 'other' field should be empty"
    );
}

#[test]
fn test_invalid_filename_case() {
    let content = r#"---
name: test-entity
description: desc
---
"#;
    let (_tmp, path) = create_fixture("test-entity", "entity.md", content);

    let result = parse_file(&path);
    assert!(
        matches!(result, Err(PersonaError::InvalidFilename(_))),
        "Expected InvalidFilename, got {:?}",
        result
    );
}

#[test]
fn test_name_mismatch() {
    let content = r#"---
name: other-name
description: desc
---
Content body"#;
    let (_tmp, path) = create_fixture("dir-name", "ENTITY.md", content);

    let result = parse_file(&path);
    match result {
        Err(PersonaError::NameMismatch {
            frontmatter_name,
            dir_name,
        }) => {
            assert_eq!(frontmatter_name, "other-name");
            assert_eq!(dir_name, "dir-name");
        }
        _ => panic!("Expected NameMismatch error, got {:?}", result),
    }
}

#[test]
fn test_invalid_name_format() {
    let content = r#"---
name: InvalidName
description: desc
---
"#;
    let (_tmp, path) = create_fixture("InvalidName", "ENTITY.md", content);

    let result = parse_file(&path);
    assert!(
        matches!(result, Err(PersonaError::InvalidNameFormat(_))),
        "Expected InvalidNameFormat, got {:?}",
        result
    );
}

#[test]
fn test_missing_frontmatter() {
    let content = r#"Just body content"#;
    let (_tmp, path) = create_fixture("test-entity", "ENTITY.md", content);

    let result = parse_file(&path);
    assert!(
        matches!(result, Err(PersonaError::MissingFrontmatter)),
        "Expected MissingFrontmatter, got {:?}",
        result
    );
}

#[test]
fn test_empty_description() {
    let content = r#"---
name: test-entity
description:
---
"#;
    let (_tmp, path) = create_fixture("test-entity", "ENTITY.md", content);

    let result = parse_file(&path);
    assert!(
        matches!(result, Err(PersonaError::EmptyDescription)),
        "Expected EmptyDescription, got {:?}",
        result
    );
}

#[test]
fn test_empty_body() {
    let content = r#"---
name: test-entity
description: valid description
---
"#;
    let (_tmp, path) = create_fixture("test-entity", "ENTITY.md", content);

    let result = parse_file(&path);
    assert!(
        matches!(result, Err(PersonaError::EmptyBody)),
        "Expected EmptyBody, got {:?}",
        result
    );
}

#[test]
fn test_nested_triples_in_content() {
    let content = r#"---
name: test-nested
description: desc
---
Here is some code:
```
---
```"#;
    let (_tmp, path) = create_fixture("test-nested", "ENTITY.md", content);

    let result = parse_file(&path);
    assert!(
        result.is_ok(),
        "Failed to parse nested triples: {:?}",
        result.err()
    );
    let entity = result.unwrap();
    assert_eq!(entity.frontmatter.name, "test-nested");
    assert_eq!(entity.body.trim(), "Here is some code:\n```\n---\n```");
}
