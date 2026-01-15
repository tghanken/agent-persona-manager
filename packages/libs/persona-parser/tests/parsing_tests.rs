use persona_parser::{PersonaError, parse_file};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

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
        let root = std::env::temp_dir().join(format!(
            "persona_parser_test_{}_{}",
            name,
            std::process::id()
        ));
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
        "---\nname: test-entity\ndescription: Test Description\n---\nBody content",
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
        "---\nname: test-entity\ndescription: desc\n---\n",
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
        "---\nname: other-name\ndescription: desc\n---\n",
    );

    let result = parse_file(&file_path);
    match result {
        Err(PersonaError::NameMismatch {
            frontmatter_name,
            dir_name,
        }) => {
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
        "---\nname: InvalidName\ndescription: desc\n---\n",
    );

    let result = parse_file(&file_path);
    assert!(matches!(result, Err(PersonaError::InvalidNameFormat(_))));
}

#[test]
fn test_missing_frontmatter() {
    let ctx = TestContext::new("missing_fm");
    let entity_dir = ctx.root.join("test-entity");
    fs::create_dir(&entity_dir).unwrap();
    let file_path = create_temp_file(&entity_dir, "ENTITY.md", "Just body content");

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
        "---\nname: test-entity\ndescription: \n---\n",
    );

    let result = parse_file(&file_path);
    assert!(matches!(result, Err(PersonaError::EmptyDescription)));
}

#[test]
fn test_nested_triples_in_content() {
    let ctx = TestContext::new("nested");
    let entity_dir = ctx.root.join("test-nested");
    fs::create_dir(&entity_dir).unwrap();
    let content =
        "---\nname: test-nested\ndescription: desc\n---\nHere is some code:\n```\n---\n```";
    let file_path = create_temp_file(&entity_dir, "ENTITY.md", content);

    let result = parse_file(&file_path);
    assert!(result.is_ok());
    let entity = result.unwrap();
    assert_eq!(entity.frontmatter.name, "test-nested");
    assert_eq!(entity.body.trim(), "Here is some code:\n```\n---\n```");
}
