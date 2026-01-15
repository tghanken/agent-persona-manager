pub fn hello() {
    println!("Hello, world!");
}

pub fn validate() {
    println!("Validating...");
}

pub fn list_files(dir: &str) {
    use std::path::Path;
    use walkdir::WalkDir;

    if !Path::new(dir).exists() {
        eprintln!("Error: Directory '{}' does not exist.", dir);
        return;
    }

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            println!("{}", entry.path().display());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        hello();
    }

    #[test]
    fn test_validate() {
        validate();
    }
}
