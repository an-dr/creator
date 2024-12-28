use crate::directory::Directory;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub struct Creator {
    directory: Directory,
    source: PathBuf,
    destination: PathBuf,
}

impl Creator {
    pub fn new(dir: PathBuf, src: PathBuf, dest: PathBuf) -> Self {
        Self {
            directory: Directory::new(dir.to_str().unwrap_or_default()),
            source: src,
            destination: dest,
        }
    }

    pub fn copy(source: &Path, destination: &Path) -> io::Result<()> {
        // Ensure the destination directory exists
        if !destination.exists() {
            fs::create_dir_all(destination)?;
        }

        // Iterate over the entries in the source directory
        for entry in fs::read_dir(source)? {
            let entry = entry?; // Handle the result of each directory entry
            let source_path = entry.path();

            // Construct the destination path
            let destination_path = destination.join(entry.file_name());

            if source_path.is_file() {
                // Copy the file to the destination
                fs::copy(&source_path, &destination_path)?;
            }
        }

        Ok(())
    }

    pub fn get_variables(&self) -> Vec<String> {
        vec![
            self.source.to_str().unwrap_or_default().to_string(),
            String::from("Var1"),
            String::from("Var2"),
            String::from("Var3"),
        ]
    }
}
