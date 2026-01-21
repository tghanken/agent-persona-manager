use crate::{EntityOrHeader, Header, PersonaError};
use persona_parser::ParsedEntity;
use quick_xml::Writer;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use std::collections::BTreeMap;
use std::io::Write;
use std::path::PathBuf;

pub fn generate_xml(
    items: &[EntityOrHeader],
    inputs: &[PathBuf],
    root_header: Option<&str>,
) -> Result<String, PersonaError> {
    let mut root = NodeRef::new();
    for item in items {
        let path = item.path();

        let mut relative_path = None;
        for input in inputs {
            if let Ok(rel) = path.strip_prefix(input) {
                relative_path = Some(rel);
                break;
            }
        }

        let rel_path = match relative_path {
            Some(p) => p,
            None => path,
        };

        let parent = rel_path.parent().ok_or_else(|| {
            PersonaError::Serialization(format!("Item has no parent directory: {:?}", path))
        })?;

        let components: Vec<String> = parent
            .iter()
            .map(|c| c.to_string_lossy().to_string())
            .filter(|s| s != ".")
            .collect();

        let mut current_node = &mut root;
        for component in components {
            current_node = current_node
                .children
                .entry(component)
                .or_insert_with(NodeRef::new);
        }

        match item {
            EntityOrHeader::Entity(e) => {
                if current_node.entity.is_some() {
                    return Err(PersonaError::Serialization(format!(
                        "Duplicate entity at path {:?}",
                        path
                    )));
                }
                current_node.entity = Some(e);
            }
            EntityOrHeader::Header(h) => {
                if current_node.header.is_some() {
                    return Err(PersonaError::Serialization(format!(
                        "Duplicate header at path {:?}",
                        path
                    )));
                }
                current_node.header = Some(h);
            }
        }
    }

    // 2. Generate XML
    let mut writer = Writer::new_with_indent(Vec::new(), b' ', 2);

    // Root element <persona-context>
    writer.write_event(Event::Start(BytesStart::new("persona-context")))?;

    if let Some(header_content) = root_header {
        let desc_elem = BytesStart::new("directions");
        writer.write_event(Event::Start(desc_elem))?;
        writer.write_event(Event::Text(BytesText::new(header_content.trim())))?;
        writer.write_event(Event::End(BytesEnd::new("directions")))?;
    }

    // Recurse
    write_node(&mut writer, &root)?;

    writer.write_event(Event::End(BytesEnd::new("persona-context")))?;

    let result = String::from_utf8(writer.into_inner())
        .map_err(|e| PersonaError::Serialization(e.to_string()))?;
    Ok(result)
}

struct NodeRef<'a> {
    children: BTreeMap<String, NodeRef<'a>>,
    entity: Option<&'a ParsedEntity>,
    header: Option<&'a Header>,
}

impl<'a> NodeRef<'a> {
    fn new() -> Self {
        Self {
            children: BTreeMap::new(),
            entity: None,
            header: None,
        }
    }
}

fn write_node<W: Write>(writer: &mut Writer<W>, node: &NodeRef) -> Result<(), PersonaError> {
    for (name, child_node) in &node.children {
        let mut elem = BytesStart::new(name);

        if let Some(entity) = child_node.entity {
            // Attribute: path
            elem.push_attribute(("path", entity.path.to_string_lossy().as_ref()));
        }

        writer.write_event(Event::Start(elem))?;

        if let Some(header) = child_node.header {
            let desc_elem = BytesStart::new("directions");
            writer.write_event(Event::Start(desc_elem.clone()))?;
            writer.write_event(Event::Text(BytesText::new(header.body.trim())))?;
            writer.write_event(Event::End(BytesEnd::new("directions")))?;
        }

        if let Some(entity) = child_node.entity {
            // Description
            let desc_elem = BytesStart::new("description");
            writer.write_event(Event::Start(desc_elem.clone()))?;
            writer.write_event(Event::Text(BytesText::from_escaped(
                &entity.frontmatter.description,
            )))?;
            writer.write_event(Event::End(BytesEnd::new("description")))?;

            // Other frontmatter fields
            write_yaml_value(writer, &entity.frontmatter.other)?;
        }

        write_node(writer, child_node)?;
        writer.write_event(Event::End(BytesEnd::new(name)))?;
    }

    Ok(())
}

fn write_yaml_value<W: Write>(
    writer: &mut Writer<W>,
    value: &serde_yaml::Value,
) -> Result<(), PersonaError> {
    match value {
        serde_yaml::Value::Mapping(map) => {
            for (k, v) in map {
                let key_str = k.as_str().ok_or_else(|| {
                    PersonaError::Serialization("YAML key must be a string".to_string())
                })?;

                // XML tags must be valid names. Assuming keys are valid.
                let elem = BytesStart::new(key_str);
                writer.write_event(Event::Start(elem.clone()))?;
                write_yaml_value(writer, v)?;
                writer.write_event(Event::End(BytesEnd::new(key_str)))?;
            }
        }
        serde_yaml::Value::String(s) => {
            writer.write_event(Event::Text(BytesText::from_escaped(s)))?;
        }
        serde_yaml::Value::Number(n) => {
            writer.write_event(Event::Text(BytesText::from_escaped(n.to_string())))?;
        }
        serde_yaml::Value::Bool(b) => {
            writer.write_event(Event::Text(BytesText::from_escaped(b.to_string())))?;
        }
        serde_yaml::Value::Sequence(seq) => {
            for item in seq {
                let elem = BytesStart::new("item");
                writer.write_event(Event::Start(elem.clone()))?;
                write_yaml_value(writer, item)?;
                writer.write_event(Event::End(BytesEnd::new("item")))?;
            }
        }
        serde_yaml::Value::Null => {} // Empty
        serde_yaml::Value::Tagged(_) => {
            return Err(PersonaError::Serialization(
                "Tagged YAML values not supported".to_string(),
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use persona_parser::{Frontmatter, ParsedEntity};
    use serde_yaml::Mapping;
    use std::path::PathBuf;

    #[test]
    fn test_generate_xml_example() {
        let inputs = vec![PathBuf::from(".")];

        let mut entity1_other = Mapping::new();
        entity1_other.insert(
            serde_yaml::Value::String("license".to_string()),
            serde_yaml::Value::String("MIT".to_string()),
        );

        let entity1 = EntityOrHeader::Entity(ParsedEntity {
            path: PathBuf::from("./skills/coding/python-helper/SKILL.md"),
            frontmatter: Frontmatter {
                name: "python-helper".to_string(),
                description: "Assists with Python coding tasks.".to_string(),
                other: serde_yaml::Value::Mapping(entity1_other),
            },
            body: "".to_string(),
            char_count: 0,
        });

        let mut entity2_other = Mapping::new();
        entity2_other.insert(
            serde_yaml::Value::String("tone".to_string()),
            serde_yaml::Value::String("Inspirational".to_string()),
        );

        let entity2 = EntityOrHeader::Entity(ParsedEntity {
            path: PathBuf::from("./personas/creative/writer/PERSONA.md"),
            frontmatter: Frontmatter {
                name: "writer".to_string(),
                description: "A creative writing assistant.".to_string(),
                other: serde_yaml::Value::Mapping(entity2_other),
            },
            body: "".to_string(),
            char_count: 0,
        });

        let items = vec![entity1, entity2];

        let xml = generate_xml(&items, &inputs, None).unwrap();

        // BTreeMap sorts keys. personas < skills.
        let expected_xml = r#"<persona-context>
  <personas>
    <creative>
      <writer path="./personas/creative/writer/PERSONA.md">
        <description>A creative writing assistant.</description>
        <tone>Inspirational</tone>
      </writer>
    </creative>
  </personas>
  <skills>
    <coding>
      <python-helper path="./skills/coding/python-helper/SKILL.md">
        <description>Assists with Python coding tasks.</description>
        <license>MIT</license>
      </python-helper>
    </coding>
  </skills>
</persona-context>"#;
        assert_eq!(xml, expected_xml);
    }

    #[test]
    fn test_generate_xml_escaping() {
        let inputs = vec![PathBuf::from(".")];
        let mut other = Mapping::new();
        other.insert(
            serde_yaml::Value::String("special".to_string()),
            serde_yaml::Value::String("<&>\"'".to_string()),
        );

        let entity = EntityOrHeader::Entity(ParsedEntity {
            path: PathBuf::from("category/entity/ENTITY.md"),
            frontmatter: Frontmatter {
                name: "entity".to_string(),
                description: "Test & check < >".to_string(),
                other: serde_yaml::Value::Mapping(other),
            },
            body: "".to_string(),
            char_count: 0,
        });

        let xml = generate_xml(&[entity], &inputs, None).unwrap();

        // Should NOT escape
        assert!(xml.contains("<&>\"'"));
        assert!(xml.contains("Test & check < >"));
    }

    #[test]
    fn test_generate_xml_header_and_children() {
        let inputs = vec![PathBuf::from(".")];

        // Represents skills/coding/HEADER.md
        let header = EntityOrHeader::Header(Header {
            path: PathBuf::from("./skills/coding/HEADER.md"),
            body: "Coding Category Description".to_string(),
        });

        // Represents skills/coding/rust/SKILL.md
        let child_entity = EntityOrHeader::Entity(ParsedEntity {
            path: PathBuf::from("./skills/coding/rust/SKILL.md"),
            frontmatter: Frontmatter {
                name: "rust".to_string(),
                description: "Rust Skill".to_string(),
                other: serde_yaml::Value::Mapping(Mapping::new()),
            },
            body: "".to_string(),
            char_count: 0,
        });

        let items = vec![header, child_entity];

        let xml = generate_xml(&items, &inputs, None).unwrap();

        // Expectation:
        // <skills>
        //   <coding>
        //     <directions>Coding Category Description</directions>
        //     <rust path="...">
        //       <description>Rust Skill</description>
        //     </rust>
        //   </coding>
        // </skills>

        assert!(xml.contains("<coding>"));
        assert!(xml.contains("<directions>Coding Category Description</directions>"));
        assert!(xml.contains("<rust path=\"./skills/coding/rust/SKILL.md\">"));
        assert!(xml.contains("<description>Rust Skill</description>"));
    }
}
