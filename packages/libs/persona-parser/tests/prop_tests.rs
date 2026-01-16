use persona_parser::parse_file;
use proptest::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

fn create_temp_file(dir: &Path, file_name: &str, content: &str) -> PathBuf {
    let file_path = dir.join(file_name);
    let mut file = fs::File::create(&file_path).unwrap();
    use std::io::Write;
    file.write_all(content.as_bytes()).unwrap();
    file_path
}

struct TestContext {
    root: PathBuf,
}

impl TestContext {
    fn new(name: &str) -> Self {
        // Use a unique name for concurrency safety if needed, though proptest might run sequentially
        let root = std::env::temp_dir().join(format!(
            "persona_parser_prop_{}_{}",
            name,
            uuid::Uuid::new_v4()
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

proptest! {
    #[test]
    fn fuzz_parser(
        name in "[a-z0-9-]{1,64}",
        description in "\\PC+", // Non-control characters, non-empty
        body in "\\PC+",
    ) {
        // We need to ensure body is not empty after trim
        if body.trim().is_empty() {
            return Ok(());
        }
        if description.trim().is_empty() {
             return Ok(());
        }

        let ctx = TestContext::new("fuzz");
        let entity_dir = ctx.root.join(&name);
        fs::create_dir(&entity_dir).unwrap();

        let content = format!(
            "---\nname: {}\ndescription: {}\n---\n{}",
            name, description, body
        );

        let file_path = create_temp_file(&entity_dir, "ENTITY.md", &content);

        let result = parse_file(&file_path);

        // It should generally succeed if the inputs conform to the regexes and logic
        // However, if description or body contain "---" on a newline, it might fail or parse weirdly.
        // We expect it to succeed OR fail with a reasonable error, not panic.

        // If it succeeds, verify content matches
        if let Ok(entity) = result {
            prop_assert_eq!(entity.frontmatter.name, name);
            // Description might have formatting differences if yaml parsed it, but usually string to string is ok
            // Note: serde_yaml might fail if description contains chars that need quoting but aren't.
        }
    }
}
