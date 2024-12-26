mod app;
mod file_operator;

use app::App;

fn test_app() {
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

    let (files, dirs) = file_operator::collect_files_and_dirs(path);

    for f in files {
        println!("File: {}", f.display());
    }

    for d in dirs {
        println!("Dir:  {}", d.display());
    }
}

fn main() {
    test_app();
}
