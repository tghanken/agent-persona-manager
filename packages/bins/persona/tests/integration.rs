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
    let root = setup_temp_dir("valid_skill");
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

    // Mock CLI arguments
    // We can't easily use Cli::parse_from because it expects string arguments and parses them.
    // But we can construct Cli directly or use parse_from.
    let cli = Cli::parse_from(["persona", "-i", root.to_str().unwrap(), "check"]);

    assert!(handle_cli(cli).is_ok());

    fs::remove_dir_all(root).unwrap();
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

    let cli = Cli::parse_from(["persona", "-i", root.to_str().unwrap(), "check"]);

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

    let cli = Cli::parse_from(["persona", "-i", root.to_str().unwrap(), "check"]);

    assert!(handle_cli(cli).is_err());

    fs::remove_dir_all(root).unwrap();
}
