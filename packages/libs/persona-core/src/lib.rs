#[tracing::instrument]
pub fn hello() {
    println!("Hello, world!");
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
}

#[tracing::instrument]
pub fn list_files(dir: &str) -> Result<Vec<std::path::PathBuf>, PersonaError> {
    use std::path::Path;
    use walkdir::WalkDir;

    if !Path::new(dir).exists() {
        tracing::error!("Directory '{}' does not exist.", dir);
        return Err(PersonaError::DirectoryNotFound(dir.to_string()));
    }

    let mut files = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            files.push(entry.path().to_path_buf());
        }
    }
    Ok(files)
}

#[tracing::instrument(skip(writer))]
pub fn print_files(
    files: &[std::path::PathBuf],
    mut writer: impl std::io::Write,
) -> std::io::Result<()> {
    for file in files {
        writeln!(writer, "{}", file.display())?;
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

        let files = list_files(temp_dir.to_str().unwrap()).unwrap();

        assert!(files.contains(&file1));
        assert!(files.contains(&file2));

        // Cleanup
        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_print_files() {
        let files = vec![
            std::path::PathBuf::from("file1.txt"),
            std::path::PathBuf::from("file2.txt"),
        ];
        let mut output = Vec::new();
        print_files(&files, &mut output).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("file1.txt"));
        assert!(output_str.contains("file2.txt"));
    }

    #[test]
    fn test_list_files_non_existent() {
        let result = list_files("non_existent_directory_xyz");
        assert!(result.is_err());
    }
}
