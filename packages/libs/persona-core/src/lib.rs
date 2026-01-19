pub mod xml;
#[tracing::instrument]
pub fn hello() {
    println!("Hello, world!");
}

pub use persona_parser::ParsedEntity;
use persona_parser::{MarkdownParser, PersonaParser as _};
use std::path::PathBuf;
use walkdir::WalkDir;

#[tracing::instrument]
pub fn collect_entities(inputs: &[PathBuf]) -> anyhow::Result<Vec<ParsedEntity>> {
    let mut errors = Vec::new();
    let mut entities = Vec::new();
    let parser = MarkdownParser;

    for dir in inputs {
        if !dir.exists() {
            let msg = format!("Directory '{}' does not exist.", dir.display());
            tracing::error!("{}", msg);
            errors.push(msg);
            continue;
        }

        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "md" {
                        let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");

                        // Check if file stem is non-empty and has NO lowercase chars
                        let is_all_caps =
                            !file_stem.is_empty() && !file_stem.chars().any(|c| c.is_lowercase());

                        if is_all_caps {
                            match parser.parse(path) {
                                Ok(entity) => entities.push(entity),
                                Err(e) => {
                                    let msg = format!("{}: {}", path.display(), e);
                                    tracing::error!("Validation error: {}", msg);
                                    errors.push(msg);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if !errors.is_empty() {
        eprintln!("Validation failed with {} errors:", errors.len());
        for err in &errors {
            eprintln!("- {}", err);
        }
        return Err(anyhow::anyhow!("Validation failed"));
    }

    Ok(entities)
}

 #[tracing::instrument]
 pub fn validate() {
     println!("Validating...");
 }
 
 #[derive(thiserror::Error, Debug)]
 pub enum PersonaError {
     #[error("Directory '{0}' does not exist")]
     DirectoryNotFound(String),
     #[error("IO error: {0}")]
     Io(#[from] std::io::Error),
    #[error("XML generation error: {0}")]
   Xml(#[from] quick_xml::Error),
   #[error("Serialization error: {0}")]
   Serialization(String),
 }


#[tracing::instrument(skip(writer))]
pub fn print_hierarchy(
    entities: &[ParsedEntity],
    inputs: &[PathBuf],
    mut writer: impl std::io::Write,
) -> std::io::Result<()> {
    struct Node {
        children: std::collections::BTreeMap<String, Node>,
    }
    impl Node {
        fn new() -> Self {
            Self {
                children: std::collections::BTreeMap::new(),
            }
        }
        fn insert(&mut self, path: &std::path::Path) {
            let mut current = self;
            for component in path.components() {
                let name = component.as_os_str().to_string_lossy().to_string();
                current = current
                    .children
                    .entry(name)
                    .or_insert_with(Node::new);
            }
        }
        fn insert_name(&mut self, name: String) {
            self.children.entry(name).or_insert_with(Node::new);
        }

        fn print(&self, writer: &mut impl std::io::Write, indent: usize) -> std::io::Result<()> {
            for (name, child) in &self.children {
                writeln!(writer, "{:indent$}{}", "", name, indent = indent * 2)?;
                child.print(writer, indent + 1)?;
            }
            Ok(())
        }
    }

    let mut root = Node::new();

    for entity in entities {
        let mut relative_path = None;
        for input in inputs {
            if let Ok(rel) = entity.path.strip_prefix(input) {
                if let Some(parent) = rel.parent() {
                    relative_path = Some(parent.to_path_buf());
                    break;
                }
            }
        }

        if let Some(path) = relative_path {
            if path.as_os_str().is_empty() {
                root.insert_name(entity.frontmatter.name.clone());
            } else {
                root.insert(&path);
            }
        } else {
             tracing::warn!("Could not determine input root for entity: {}", entity.path.display());
        }
    }

    root.print(&mut writer, 0)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_hello() {
        hello();
    }

    #[test]
    fn test_print_hierarchy() {
        use persona_parser::Frontmatter;

        let inputs = vec![PathBuf::from("/root")];
        let entities = vec![
            ParsedEntity {
                path: PathBuf::from("/root/cat/sub/ent/ENT.md"),
                frontmatter: Frontmatter {
                    name: "ent".to_string(),
                    description: "".to_string(),
                    other: Default::default(),
                },
                body: "".to_string(),
            },
            ParsedEntity {
                path: PathBuf::from("/root/cat/other/OTHER.md"),
                frontmatter: Frontmatter {
                    name: "other".to_string(),
                    description: "".to_string(),
                    other: Default::default(),
                },
                body: "".to_string(),
            },
        ];

        let mut output = Vec::new();
        print_hierarchy(&entities, &inputs, &mut output).unwrap();
        let output_str = String::from_utf8(output).unwrap();

        println!("{}", output_str);

        let expected_output = "cat\n  other\n  sub\n    ent\n";
        assert_eq!(output_str, expected_output);
    }

    #[test]
    fn test_print_hierarchy_root_entity() {
        use persona_parser::Frontmatter;

        let inputs = vec![PathBuf::from("/root/specs")];
        let entities = vec![
            ParsedEntity {
                path: PathBuf::from("/root/specs/SPECS.md"),
                frontmatter: Frontmatter {
                    name: "specs".to_string(),
                    description: "".to_string(),
                    other: Default::default(),
                },
                body: "".to_string(),
            },
        ];

        let mut output = Vec::new();
        print_hierarchy(&entities, &inputs, &mut output).unwrap();
        let output_str = String::from_utf8(output).unwrap();

        println!("{}", output_str);

        assert!(output_str.contains("specs\n"));
    }

    #[test]
    fn test_collect_entities() {
        // Test with empty inputs, should pass
        assert!(collect_entities(&[]).is_ok());
    }
 
     #[test]
     fn test_list_files() {
        // Create a temporary directory structure
        let temp_dir = std::env::temp_dir().join("persona_test_collect_entities");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
        fs::create_dir_all(&temp_dir).unwrap();

        // Create a valid entity
        let entity_dir = temp_dir.join("entity1");
        fs::create_dir(&entity_dir).unwrap();
        let entity_file = entity_dir.join("ENTITY1.md");
        let content = "---\nname: entity1\ndescription: Test entity\n---\nBody content";
        fs::write(&entity_file, content).unwrap();

        // Create an invalid entity (name mismatch)
        let invalid_entity_dir = temp_dir.join("entity2");
        fs::create_dir(&invalid_entity_dir).unwrap();
        let invalid_entity_file = invalid_entity_dir.join("ENTITY2.md");
        let invalid_content = "---\nname: wrongname\ndescription: Test entity\n---\nBody content";
        fs::write(&invalid_entity_file, invalid_content).unwrap();

        // Test
        let inputs = vec![temp_dir.clone()];
        let result = collect_entities(&inputs);

        // Should fail because of invalid entity
        assert!(result.is_err());

        // Remove invalid entity
        fs::remove_dir_all(&invalid_entity_dir).unwrap();

        // Test again
        let result = collect_entities(&inputs);
        assert!(result.is_ok());
        let entities = result.unwrap();
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].frontmatter.name, "entity1");

        // Cleanup
        fs::remove_dir_all(&temp_dir).unwrap();
    }
}
