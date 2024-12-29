mod creator;
mod directory_analyzer;
mod environment;
mod tui;

use directory_analyzer::DirectoryAnalyzer;

fn test_app() {
    // let cwd = std::env::current_dir().expect("Failed to get current directory");

    // let mut path = cwd.clone();

    // // app.start();

    // println!("\n");

    // let (files, dirs) = creator_operations::collect_files_and_dirs(path);

    // for f in files {
    //     println!("File: {}", f.display());
    // }

    // for d in dirs {
    //     println!("Dir:  {}", d.display());
    // }
    let mut dir = DirectoryAnalyzer::new(&environment::get_storage_path());

    let (files, dirs) = dir.get_items();
    for f in files {
        println!("File: {}", f.file_name().unwrap().to_str().unwrap());
    }

    for d in dirs {
        println!("Dir:  {}", d.file_name().unwrap().to_str().unwrap());
    }
}

fn run() {
    tui::run();
}

fn main() {
    run();
    // test_app();
}
