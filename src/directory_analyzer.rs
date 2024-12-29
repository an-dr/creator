use regex::Regex;
use std::{collections::HashSet, fs, path::PathBuf};
use walkdir::WalkDir;

const DEFAULT_SEARCH_PATTERN: &str = r"#var_(.*?)#";

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

    fn search_and_append(text: &str, vars_to_append: &mut HashSet<String>) {
        let re = Regex::new(DEFAULT_SEARCH_PATTERN)
            .expect("Matching pattern must be accepted");
        for mat in re.find_iter(text) {
            vars_to_append.insert(mat.as_str().to_string());
            println!("Found: {}", mat.as_str());
        }
    }

    pub fn scan_variables(&self) -> HashSet<String> {
        let mut vars: HashSet<String> = HashSet::new();

        let (files, dirs) = self.get_items_recursively();

        for f in files {
            // Scan file names
            Self::search_and_append(f.file_name().unwrap().to_str().unwrap(), &mut vars);
            // Scan file content
            let content = fs::read_to_string(&f).unwrap();
            Self::search_and_append(&content, &mut vars);
        }

        for d in dirs {
            // Scan names
            Self::search_and_append(d.file_name().unwrap().to_str().unwrap(), &mut vars);
        }

        vars
    }
}
