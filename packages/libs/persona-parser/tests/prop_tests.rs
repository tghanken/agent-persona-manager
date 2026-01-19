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
        extra_fields in proptest::collection::hash_map("[a-z]+", "\\PC+", 0..10),
    ) {
        if body.trim().is_empty() || description.trim().is_empty() {
            return Ok(());
        }

        let ctx = TestContext::new("fuzz");
        let entity_dir = ctx.root.join(&name);
        fs::create_dir(&entity_dir).unwrap();

        // Construct valid YAML using serde to avoid syntax errors from random content
        let mut frontmatter = serde_yaml::Mapping::new();
        frontmatter.insert(serde_yaml::Value::String("name".to_string()), serde_yaml::Value::String(name.clone()));
        frontmatter.insert(serde_yaml::Value::String("description".to_string()), serde_yaml::Value::String(description.clone()));

        for (k, v) in &extra_fields {
            frontmatter.insert(serde_yaml::Value::String(k.clone()), serde_yaml::Value::String(v.clone()));
        }

        let frontmatter_str = serde_yaml::to_string(&frontmatter).unwrap();
        // serde_yaml includes the "---" separator usually? No, just the content.
        // Wait, to_string might produce "---" at start if it thinks it's a document.
        // Let's trim it and wrap it ourselves to be sure we match our parser expectation
        let frontmatter_clean = frontmatter_str.trim_start_matches("---\n").trim();

        let content = format!(
            "---\n{}\n---\n{}",
            frontmatter_clean, body
        );

        let file_path = create_temp_file(&entity_dir, "ENTITY.md", &content);

        let parser = MarkdownParser;
        let result = parser.parse(&file_path);

        if let Ok(entity) = result {
            prop_assert_eq!(entity.frontmatter.name, name);
            for (k, _v) in extra_fields {
                prop_assert!(entity.frontmatter.other.get(&k).is_some(), "Field {} missing from other", k);
            }
        }
    }
}
