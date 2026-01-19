use crate::PersonaError;
use persona_parser::ParsedEntity;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::PathBuf;

pub fn generate_xml(
    entities: &[ParsedEntity],
    inputs: &[PathBuf],
) -> Result<String, PersonaError> {
    let mut root = NodeRef::new();
    for entity in entities {
        let path = &entity.path;

        let mut relative_path = None;
        for input in inputs {
            if let Ok(rel) = path.strip_prefix(input) {
                relative_path = Some(rel);
                break;
            }
        }

        let rel_path = match relative_path {
            Some(p) => p,
            None => path.as_path(),
        };

        let parent = rel_path.parent().ok_or_else(|| {
            PersonaError::Serialization(format!("Entity has no parent directory: {:?}", path))
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

        if current_node.entity.is_some() {
            return Err(PersonaError::Serialization(format!(
                "Duplicate entity at path {:?}",
                path
            )));
        }
        current_node.entity = Some(entity);
    }

    // 2. Generate XML
    let mut writer = Writer::new_with_indent(Vec::new(), b' ', 2);

    // Root element <persona-context>
    writer.write_event(Event::Start(BytesStart::new("persona-context")))?;

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
}

impl<'a> NodeRef<'a> {
    fn new() -> Self {
        Self {
            children: BTreeMap::new(),
            entity: None,
        }
    }
}

fn write_node<W: Write>(writer: &mut Writer<W>, node: &NodeRef) -> Result<(), PersonaError> {
    for (name, child_node) in &node.children {
        if let Some(entity) = child_node.entity {
            // It's an entity.
            let mut elem = BytesStart::new(name);
            // Attribute: path
            elem.push_attribute(("path", entity.path.to_string_lossy().as_ref()));

            writer.write_event(Event::Start(elem))?;

            // Description
            let desc_elem = BytesStart::new("description");
            writer.write_event(Event::Start(desc_elem.clone()))?;
            writer.write_event(Event::Text(BytesText::new(&entity.frontmatter.description)))?;
            writer.write_event(Event::End(BytesEnd::new("description")))?;

            // Other frontmatter fields
            write_yaml_value(writer, &entity.frontmatter.other)?;

            writer.write_event(Event::End(BytesEnd::new(name)))?;
        } else {
            let elem = BytesStart::new(name);
            writer.write_event(Event::Start(elem.clone()))?;
            write_node(writer, child_node)?;
            writer.write_event(Event::End(BytesEnd::new(name)))?;
        }
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
            writer.write_event(Event::Text(BytesText::new(s)))?;
        }
        serde_yaml::Value::Number(n) => {
            writer.write_event(Event::Text(BytesText::new(&n.to_string())))?;
        }
        serde_yaml::Value::Bool(b) => {
            writer.write_event(Event::Text(BytesText::new(&b.to_string())))?;
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

        let entity1 = ParsedEntity {
            path: PathBuf::from("./skills/coding/python-helper/SKILL.md"),
            frontmatter: Frontmatter {
                name: "python-helper".to_string(),
                description: "Assists with Python coding tasks.".to_string(),
                other: serde_yaml::Value::Mapping(entity1_other),
            },
            body: "".to_string(),
        };

        let mut entity2_other = Mapping::new();
        entity2_other.insert(
            serde_yaml::Value::String("tone".to_string()),
            serde_yaml::Value::String("Inspirational".to_string()),
        );

        let entity2 = ParsedEntity {
            path: PathBuf::from("./personas/creative/writer/PERSONA.md"),
            frontmatter: Frontmatter {
                name: "writer".to_string(),
                description: "A creative writing assistant.".to_string(),
                other: serde_yaml::Value::Mapping(entity2_other),
            },
            body: "".to_string(),
        };

        let entities = vec![entity1, entity2];

        let xml = generate_xml(&entities, &inputs).unwrap();

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

        let entity = ParsedEntity {
            path: PathBuf::from("category/entity/ENTITY.md"),
            frontmatter: Frontmatter {
                name: "entity".to_string(),
                description: "Test & check < >".to_string(),
                other: serde_yaml::Value::Mapping(other),
            },
            body: "".to_string(),
        };

        let xml = generate_xml(&[entity], &inputs).unwrap();

        // Should be escaped
        assert!(xml.contains("&lt;&amp;&gt;&quot;&apos;"));
        assert!(xml.contains("Test &amp; check &lt; &gt;"));
    }
}
