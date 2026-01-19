use persona_parser::{MarkdownParser, PersonaParser};

// Integration-style test for full flow using the Parser trait
// We'll use the inlined content approach to be Nix-safe, but now we test the trait directly.
// We need to write to disk because the trait takes a path and reads it.

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
    assert!(
        entity
            .frontmatter
            .other
            .as_mapping()
            .is_none_or(|m| m.is_empty())
    );
}
