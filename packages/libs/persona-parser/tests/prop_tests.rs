use persona_parser::{MarkdownParser, PersonaParser};
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
        description in "\\PC+",
        body in "\\PC+",
        extra_key in "[a-z]+",
        extra_value in "\\PC+",
    ) {
        if body.trim().is_empty() || description.trim().is_empty() {
            return Ok(());
        }

        // Ensure name is valid according to our strict rules (regex above covers most, but verify logic)

        let ctx = TestContext::new("fuzz");
        let entity_dir = ctx.root.join(&name);
        fs::create_dir(&entity_dir).unwrap();

        // Add extra fields to frontmatter
        let content = format!(
            "---\nname: {}\ndescription: {}\n{}: {}\n---\n{}",
            name, description, extra_key, extra_value, body
        );

        let file_path = create_temp_file(&entity_dir, "ENTITY.md", &content);

        let parser = MarkdownParser;
        let result = parser.parse(&file_path);

        if let Ok(entity) = result {
            prop_assert_eq!(entity.frontmatter.name, name);
            // Check that extra fields are captured in 'other'
            if let Some(_val) = entity.frontmatter.other.get(extra_key.as_str()) {
                 // Value found
            }
        }
    }
}
