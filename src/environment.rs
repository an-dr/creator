use std::env;

pub const TEMPLATES_PATH_VAR_NAME: &str = "CREATOR_STORAGE";
pub const DEFAULT_TEMPLATE_PATH: &str = "D:/dev-templates/templates";

// pub fn get_variables(template_dir: &str) -> Vec<String> {
//     vec![
//         String::from(template_dir),
//         String::from("Var1"),
//         String::from("Var2"),
//         String::from("Var3"),
//     ]
// }

// pub fn print_files_recursively(path: PathBuf) {
//     // Walk the directory recursively
//     for entry in WalkDir::new(path) {
//         match entry {
//             Ok(e) => {
//                 // Check if the entry is a file and print its path
//                 if e.file_type().is_file() {
//                     println!("File: {}", e.path().display());
//                 } else {
//                     println!("Dir:  {}", e.path().display());
//                 }
//             }
//             Err(e) => {
//                 eprintln!("Error reading entry: {}", e);
//             }
//         }
//     }
// }

pub fn get_current_working_directory() -> String {
    env::current_dir()
        .expect("CWD is not accessible!")
        .to_str()
        .expect("Cannot covert to String")
        .to_string()
}

pub fn get_storage_path() -> String {
    std::env::var(TEMPLATES_PATH_VAR_NAME)
        .unwrap_or_else(|_| DEFAULT_TEMPLATE_PATH.to_string())
}
