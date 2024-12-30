mod app_config;
mod creator;
mod directory_analyzer;
mod environment;
mod tui;
use directory_analyzer::DirectoryAnalyzer;
use log::{info};
use simplelog::*;
use std::fs::File;

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
    // // Set up logging to a file
    // let log_file = File::create("creator.log").expect("Failed to creqate log file");
    // CombinedLogger::init(vec![WriteLogger::new(
    //     LevelFilter::Info,
    //     Config::default(),
    //     log_file,
    // )])
    // .expect("Failed to initialize logger");

    run();
    // test_app();
}
