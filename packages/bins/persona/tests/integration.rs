use clap::Parser;
use persona::{Cli, handle_cli};
use std::fs;
use std::path::PathBuf;

fn setup_temp_dir(name: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    dir.push(format!("persona_integration_test_{}", name));
    if dir.exists() {
        fs::remove_dir_all(&dir).unwrap();
    }
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn test_check_valid_skill() {
    let temp = setup_temp_dir("valid_skill");
    let root = temp.join("inputs");
    fs::create_dir(&root).unwrap();

    let skill_dir = root.join("skills/coding/rust");
    fs::create_dir_all(&skill_dir).unwrap();

    let skill_file = skill_dir.join("SKILL.md");
    let content = r#"---
name: rust
description: Rust programming skill
---
Use rust for systems programming.
"#;
    fs::write(&skill_file, content).unwrap();

    // Generate expected AGENTS.md content
    let agents_file = temp.join("AGENTS.md");
    // Note: generate_xml uses entity.path for the path attribute.
    // Since we pass absolute path as input, entity.path is absolute.
    // Ideally we should use relative paths, but for this test we match what generate_xml produces.
    let expected_xml = format!(
        r#"<persona-context>
  <skills>
    <coding>
      <rust path="{}">
        <description>Rust programming skill</description>
      </rust>
    </coding>
  </skills>
</persona-context>"#,
        skill_file.to_string_lossy()
    );
    fs::write(&agents_file, &expected_xml).unwrap();

    // Mock CLI arguments
    let cli = Cli::parse_from([
        "persona",
        "-i",
        root.to_str().unwrap(),
        "check",
        "--agents-file",
        agents_file.to_str().unwrap(),
    ]);

    handle_cli(cli).unwrap();

    fs::remove_dir_all(temp).unwrap();
}

#[test]
fn test_check_fails_on_outdated_agents_md() {
    let temp = setup_temp_dir("outdated_check");
    let root = temp.join("inputs");
    fs::create_dir(&root).unwrap();

    let skill_dir = root.join("skills/coding/rust");
    fs::create_dir_all(&skill_dir).unwrap();

    let skill_file = skill_dir.join("SKILL.md");
    let content = r#"---
name: rust
description: Rust programming skill
---
Use rust for systems programming.
"#;
    fs::write(&skill_file, content).unwrap();

    // Write incorrect AGENTS.md
    let agents_file = temp.join("AGENTS.md");
    fs::write(&agents_file, "INVALID CONTENT").unwrap();

    let cli = Cli::parse_from([
        "persona",
        "-i",
        root.to_str().unwrap(),
        "check",
        "--agents-file",
        agents_file.to_str().unwrap(),
    ]);

    assert!(handle_cli(cli).is_err());

    fs::remove_dir_all(temp).unwrap();
}

#[test]
fn test_check_fails_on_missing_agents_md() {
    let temp = setup_temp_dir("missing_check");
    let root = temp.join("inputs");
    fs::create_dir(&root).unwrap();

    let skill_dir = root.join("skills/coding/rust");
    fs::create_dir_all(&skill_dir).unwrap();

    let skill_file = skill_dir.join("SKILL.md");
    let content = r#"---
name: rust
description: Rust programming skill
---
Use rust for systems programming.
"#;
    fs::write(&skill_file, content).unwrap();

    let agents_file = temp.join("AGENTS.md");
    // Do NOT write AGENTS.md

    let cli = Cli::parse_from([
        "persona",
        "-i",
        root.to_str().unwrap(),
        "check",
        "--agents-file",
        agents_file.to_str().unwrap(),
    ]);

    assert!(handle_cli(cli).is_err());

    fs::remove_dir_all(temp).unwrap();
}

#[test]
fn test_build_valid_skill() {
    let root = setup_temp_dir("build_valid");
    let skill_dir = root.join("skills/coding/rust");
    fs::create_dir_all(&skill_dir).unwrap();

    let skill_file = skill_dir.join("SKILL.md");
    let content = r#"---
name: rust
description: Rust programming skill
---
Use rust for systems programming.
"#;
    fs::write(&skill_file, content).unwrap();

    let output_dir = root.join("output");

    let cli = Cli::parse_from([
        "persona",
        "-i",
        root.to_str().unwrap(),
        "build",
        "-o",
        output_dir.to_str().unwrap(),
    ]);

    assert!(handle_cli(cli).is_ok());

    assert!(output_dir.join("skills/coding/rust/SKILL.md").exists());
    assert!(std::path::Path::new("AGENTS.md").exists());

    // Clean up
    fs::remove_dir_all(root).unwrap();
    let _ = fs::remove_file("AGENTS.md");
}

#[test]
fn test_check_invalid_skill_missing_frontmatter() {
    let root = setup_temp_dir("invalid_skill_mf");
    let skill_dir = root.join("skills/coding/rust");
    fs::create_dir_all(&skill_dir).unwrap();

    let skill_file = skill_dir.join("SKILL.md");
    let content = r#"
Use rust for systems programming.
"#;
    fs::write(&skill_file, content).unwrap();

    let agents_file = root.join("AGENTS.md");
    let cli = Cli::parse_from([
        "persona",
        "-i",
        root.to_str().unwrap(),
        "check",
        "--agents-file",
        agents_file.to_str().unwrap(),
    ]);

    assert!(handle_cli(cli).is_err());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn test_check_invalid_skill_name_mismatch() {
    let root = setup_temp_dir("name_mismatch");
    let skill_dir = root.join("skills/coding/rust");
    fs::create_dir_all(&skill_dir).unwrap();

    let skill_file = skill_dir.join("SKILL.md");
    let content = r#"---
name: python
description: Rust programming skill
---
Use rust.
"#;
    fs::write(&skill_file, content).unwrap();

    let agents_file = root.join("AGENTS.md");
    let cli = Cli::parse_from([
        "persona",
        "-i",
        root.to_str().unwrap(),
        "check",
        "--agents-file",
        agents_file.to_str().unwrap(),
    ]);

    assert!(handle_cli(cli).is_err());

    fs::remove_dir_all(root).unwrap();
}
