mod app;

use app::App;

mod replacer {

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
}

fn main() {
    // std::env::set_var(App::, "VALUE");
    let cwd = std::env::current_dir().expect("Failed to get current directory");

    let mut path = cwd.clone();
    path.push("tests");
    path.push("assets");
    path.push("test_input");

    println!("Test Path: {:?}", path.display());

    let mut app = App::new();
    app.start();

    println!("\n");
    
    let (files, dirs) = replacer::collect_files_and_dirs(path);
    
    for f in files{
        println!("File: {}", f.display());
    }
    
    for d in dirs{
        println!("Dir:  {}", d.display());
    }
    
    
        
    
}
