use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use persona_core::{collect_entities, print_hierarchy, xml::generate_xml};
use std::fs;
use tempfile::TempDir;

fn setup_env(num_files: usize) -> TempDir {
    let temp_dir = tempfile::tempdir().unwrap();
    let root = temp_dir.path();

    let content_template = r#"---
name: skill-{i}
description: Dummy skill for benchmarking
---
# Skill {i}

This is a dummy skill.
"#;

    for i in 0..num_files {
        // Group by 100 in subdirectories to keep directory listing size manageable
        let subdir_index = i / 100;
        let group_dir = root.join(format!("group_{}", subdir_index));
        if !group_dir.exists() {
            fs::create_dir(&group_dir).unwrap();
        }

        // Each skill must be in a directory matching its name
        let skill_name = format!("skill-{}", i);
        let skill_dir = group_dir.join(&skill_name);
        fs::create_dir(&skill_dir).unwrap();

        // ALL CAPS filename required for recognition
        let filepath = skill_dir.join("SKILL.md");
        let file_content = content_template.replace("{i}", &i.to_string());

        fs::write(filepath, file_content).unwrap();
    }

    temp_dir
}

fn bench_commands(c: &mut Criterion) {
    let file_counts = [10, 100, 1000, 10000];

    let mut group = c.benchmark_group("commands");
    // Increase sample size and measurement time for larger inputs
    group.sample_size(10);
    group.measurement_time(std::time::Duration::from_secs(10));

    for &count in &file_counts {
        println!("Setting up environment with {} files...", count);
        let temp_dir = setup_env(count);
        let inputs = vec![temp_dir.path().to_path_buf()];
        println!("Setup complete for {}.", count);

        group.bench_with_input(BenchmarkId::new("check", count), &inputs, |b, inputs| {
            b.iter(|| {
                collect_entities(inputs).unwrap();
            })
        });

        group.bench_with_input(BenchmarkId::new("list", count), &inputs, |b, inputs| {
            b.iter(|| {
                let entities = collect_entities(inputs).unwrap();
                print_hierarchy(&entities, inputs, std::io::sink()).unwrap();
            })
        });

        group.bench_with_input(BenchmarkId::new("build", count), &inputs, |b, inputs| {
            b.iter(|| {
                let entities = collect_entities(inputs).unwrap();
                let _xml = generate_xml(&entities, inputs).unwrap();
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_commands);
criterion_main!(benches);
