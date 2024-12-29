use std::collections::HashMap;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub struct Creator {
    source: PathBuf,
    source_variable_values: HashMap<String, String>,
    destination: PathBuf,
}

impl Creator {
    pub fn new(src: &Path, dest: &Path) -> Self {
        let mut s = Self {
            source: PathBuf::from(src),
            source_variable_values: HashMap::new(),
            destination: PathBuf::new(),
        };

        let src_dir_name = src.file_name().expect("Cannot read source file name");
        s.destination = PathBuf::from(dest).join(src_dir_name);

        s
    }

    pub fn get_source(&self) -> &Path {
        &self.source
    }

    pub fn get_destination(&self) -> &Path {
        &self.destination
    }

    pub fn create(&self) -> io::Result<()> {
        // Ensure the destination directory exists
        if !self.destination.exists() {
            fs::create_dir_all(self.destination.clone())?;
        }

        // Iterate over the entries in the source directory
        for entry in fs::read_dir(self.source.clone())? {
            let entry = entry?; // Handle the result of each directory entry
            let source_path = entry.path();

            // Construct the destination path
            let destination_path = self.destination.join(entry.file_name());

            if source_path.is_file() {
                // Copy the file to the destination
                fs::copy(&source_path, &destination_path)?;
            }
        }

        Ok(())
    }

    pub fn set_var_values(&mut self, var_values: &HashMap<String, String>) {
        self.source_variable_values = var_values.clone();
    }

    pub fn get_var_values(&self) -> &HashMap<String, String> {
        &self.source_variable_values
    }
}
