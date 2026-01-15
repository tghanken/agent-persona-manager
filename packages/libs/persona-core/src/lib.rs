#[tracing::instrument]
pub fn hello() {
    println!("Hello, world!");
}

#[tracing::instrument]
pub fn validate() {
    println!("Validating...");
}

#[tracing::instrument(skip(writer))]
pub fn list_files(dir: &str, mut writer: impl std::io::Write) -> std::io::Result<()> {
    use std::path::Path;
    use walkdir::WalkDir;

    if !Path::new(dir).exists() {
        tracing::error!("Directory '{}' does not exist.", dir);
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Directory '{}' does not exist.", dir),
        ));
    }

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            writeln!(writer, "{}", entry.path().display())?;
        }
    }
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
    fn test_validate() {
        validate();
    }

    #[test]
    fn test_list_files() {
        // Create a temporary directory structure
        let temp_dir = std::env::temp_dir().join("persona_test_list_files");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
        fs::create_dir_all(&temp_dir).unwrap();
        let file1 = temp_dir.join("file1.txt");
        let sub_dir = temp_dir.join("sub");
        let file2 = sub_dir.join("file2.txt");

        fs::write(&file1, "content").unwrap();
        fs::create_dir(&sub_dir).unwrap();
        fs::write(&file2, "content").unwrap();

        let mut output = Vec::new();
        list_files(temp_dir.to_str().unwrap(), &mut output).unwrap();

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains(file1.to_str().unwrap()));
        assert!(output_str.contains(file2.to_str().unwrap()));

        // Cleanup
        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_list_files_non_existent() {
        let mut output = Vec::new();
        let result = list_files("non_existent_directory_xyz", &mut output);
        assert!(result.is_err());
    }
}
