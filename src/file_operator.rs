use std::path::PathBuf;
use walkdir::WalkDir;

pub fn print_files_recursively(path: PathBuf) {
    // Walk the directory recursively
    for entry in WalkDir::new(path) {
        match entry {
            Ok(e) => {
                // Check if the entry is a file and print its path
                if e.file_type().is_file() {
                    println!("File: {}", e.path().display());
                } else {
                    println!("Dir:  {}", e.path().display());
                }
            }
            Err(e) => {
                eprintln!("Error reading entry: {}", e);
            }
        }
    }
}

pub fn collect_files_and_dirs(path: PathBuf) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut files = Vec::new();
    let mut dirs = Vec::new();

    // Walk the directory recursively
    for entry in WalkDir::new(path) {
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
