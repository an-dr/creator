use std::path::PathBuf;
use walkdir::WalkDir;

pub struct DirectoryAnalyzer {
    path: PathBuf,
}

impl DirectoryAnalyzer {
    pub fn new(path: &str) -> DirectoryAnalyzer {
        DirectoryAnalyzer {
            path: PathBuf::from(path),
        }
    }

    pub fn get_nested_directory(&self, name: &str) -> Option<DirectoryAnalyzer> {
        let mut path = self.path.clone();
        path.push(name);
        if path.is_dir() {
            Some(DirectoryAnalyzer::new(path.to_str().unwrap()))
        } else {
            None
        }
    }

    pub fn get_items(&self) -> (Vec<PathBuf>, Vec<PathBuf>) {
        let mut files = Vec::new();
        let mut dirs = Vec::new();

        for entry in std::fs::read_dir(&self.path).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                files.push(entry.path());
            } else if entry.file_type().unwrap().is_dir() {
                dirs.push(entry.path());
            }
        }

        (files, dirs)
    }

    pub fn get_items_recursively(&self) -> (Vec<PathBuf>, Vec<PathBuf>) {
        let mut files = Vec::new();
        let mut dirs = Vec::new();

        for entry in WalkDir::new(&self.path) {
            match entry {
                Ok(e) => {
                    if e.file_type().is_file() {
                        files.push(e.path().to_path_buf());
                    } else if e.file_type().is_dir() {
                        dirs.push(e.path().to_path_buf());
                    }
                }
                Err(e) => {
                    eprintln!("Error reading entry: {}", e);
                }
            }
        }

        (files, dirs)
    }

    pub fn scan_variables(&self) -> Vec<String> {
        vec![
            self.path.to_str().unwrap_or_default().to_string(),
            String::from("Var1"),
            String::from("Var2"),
            String::from("Var3"),
        ]
    }

}
