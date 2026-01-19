use crate::PersonaError;
use persona_parser::ParsedEntity;
use quick_xml::Writer;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use std::collections::BTreeMap;
use std::io::Write;

pub fn generate_xml(entities: &[ParsedEntity]) -> Result<String, PersonaError> {
    let mut root = NodeRef::new();
    for entity in entities {
        let path = &entity.path;
        let parent = path.parent().ok_or_else(|| {
            PersonaError::Serialization(format!("Entity has no parent directory: {:?}", path))
        })?;

        let mut components: Vec<String> = parent
            .iter()
            .map(|c| c.to_string_lossy().to_string())
            .collect();

        if components.first().map(|s| s == ".").unwrap_or(false) {
            components.remove(0);
        }

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
    // If this node has an entity, it's a leaf in terms of content, but it might still have children?
    // Spec: "Nesting: Entity directories may contain one level of subdirectories (e.g., scripts/, assets/) which are not parsed for further entities. Deeply nested directories are not allowed."
    // "Leaf Elements: The tag name of the leaf element is the `name` of the entity."
    // So if a node has an entity, it shouldn't have children that are also entities (based on "not parsed for further entities").
    // But in our tree, `children` represent subdirectories.
    // If `node.entity` is Some, then this node corresponds to the directory `.../entity-name/`.
    // The spec says "The XML structure mirrors the directory category/subcategory hierarchy."

    // Case 1: Intermediate directory (Category/Subcategory).
    // It has children, but no entity.
    // XML: <category> ... children ... </category>

    // Case 2: Entity directory.
    // It has an entity.
    // XML: <entity-name ...> ... content ... </entity-name>

    // We iterate over children map to ensure order (BTreeMap).

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
            // It's a category.
            // Only write if it has children (which it must, otherwise why is it in the tree? Well, it could be a leaf dir with no entity? No, we built tree from entities).
            // Wait, if we built tree from entities, every leaf in the tree MUST be an entity.
            // Intermediate nodes are just path components.

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
            // How to represent sequences?
            // Spec example doesn't show sequence.
            // Usually repeated tags or <item>.
            // Let's assume standard XML serialization: just text content joined? Or error?
            // "Child Elements: All fields found in the YAML frontmatter of the entity."
            // If I have `tags: [a, b]`, XML might be `<tags><a></a><b></b></tags>` (if keys are implicit?) NO.
            // Usually `<tags><item>a</item><item>b</item></tags>` or just repeated `<tags>a</tags><tags>b</tags>` if parent handles it.
            // But we are inside `write_yaml_value` which is called *inside* the tag for the key.
            // So if we are in `<tags>`, and value is sequence.
            // Maybe just stringify?
            // Let's assume for now simple scalar values as per example.
            // If complex, let's output JSON-like structure or just items.
            // Let's use <item> for now.
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
        // Input structure:
        // -   `skills/coding/python-helper/SKILL.md` (name: `python-helper`)
        // -   `personas/creative/writer/PERSONA.md` (name: `writer`)

        let mut entity1_other = Mapping::new();
        entity1_other.insert(
            serde_yaml::Value::String("license".to_string()),
            serde_yaml::Value::String("MIT".to_string()),
        );

        let entity1 = ParsedEntity {
            path: PathBuf::from("skills/coding/python-helper/SKILL.md"),
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
            path: PathBuf::from("personas/creative/writer/PERSONA.md"),
            frontmatter: Frontmatter {
                name: "writer".to_string(),
                description: "A creative writing assistant.".to_string(),
                other: serde_yaml::Value::Mapping(entity2_other),
            },
            body: "".to_string(),
        };

        let entities = vec![entity1, entity2];

        let xml = generate_xml(&entities).unwrap();

        println!("{}", xml);

        // Assertions
        assert!(xml.contains("<persona-context>"));
        assert!(xml.contains("<skills>"));
        assert!(xml.contains("<coding>"));
        assert!(xml.contains("<python-helper path=\"skills/coding/python-helper/SKILL.md\">"));
        assert!(xml.contains("<description>Assists with Python coding tasks.</description>"));
        assert!(xml.contains("<license>MIT</license>"));
        assert!(xml.contains("</python-helper>"));
        assert!(xml.contains("</coding>"));
        assert!(xml.contains("</skills>"));

        assert!(xml.contains("<personas>"));
        assert!(xml.contains("<creative>"));
        assert!(xml.contains("<writer path=\"personas/creative/writer/PERSONA.md\">"));
        assert!(xml.contains("<description>A creative writing assistant.</description>"));
        assert!(xml.contains("<tone>Inspirational</tone>"));
        assert!(xml.contains("</writer>"));
        assert!(xml.contains("</creative>"));
        assert!(xml.contains("</personas>"));
        assert!(xml.contains("</persona-context>"));
    }

    #[test]
    fn test_generate_xml_escaping() {
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

        let xml = generate_xml(&[entity]).unwrap();

        assert!(xml.contains("&lt;&amp;&gt;&quot;&apos;"));
        assert!(xml.contains("Test &amp; check &lt; &gt;"));
    }
}
