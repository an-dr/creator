use crate::app_config;
use crate::directory_analyzer::DirectoryAnalyzer;
use log::debug;
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

        let dir_an = DirectoryAnalyzer::new(&self.source);
        let (files, dirs) = dir_an.get_items_recursively();

        for d in dirs {
            // Get the destination path
            let rel_path = d
                .strip_prefix(&self.source)
                .expect("The prefix should be the same");
            let mut dest_path = self.destination.clone();
            dest_path.push(rel_path);

            // Replace the variables in the path
            let mut dest_path_str = dest_path
                .to_str()
                .expect("The path should be valid")
                .to_string();
            for (var_name, var_value) in &self.source_variable_values {
                let new_var_name = format!(
                    "{}{}{}",
                    app_config::TEMPLATE_VAR_PREFIX,
                    var_name,
                    app_config::TEMPLATE_VAR_SUFFIX
                );
                dest_path_str = dest_path_str.replace(&new_var_name, var_value);
            }

            // Create the directory
            fs::create_dir_all(&dest_path_str)?;
        }

        for f in files {
            // Get the destination path
            let rel_path = f
                .strip_prefix(&self.source)
                .expect("The prefix should be the same");
            debug!("Relative path: {:?}", rel_path);
            let mut dest_path = self.destination.clone();
            dest_path.push(rel_path);
            debug!("Source path: {:?}", f);

            // Replace the variables in the path
            let mut dest_path_str = dest_path
                .to_str()
                .expect("The path should be valid")
                .to_string();
            for (var_name, var_value) in &self.source_variable_values {
                let new_var_name = format!(
                    "{}{}{}",
                    app_config::TEMPLATE_VAR_PREFIX,
                    var_name,
                    app_config::TEMPLATE_VAR_SUFFIX
                );
                dest_path_str = dest_path_str.replace(&new_var_name, var_value);
            }
            debug!("Dest path: {:?}\n", dest_path_str);

            //Copy the file
            fs::copy(&f, &dest_path_str)?;

            // Replace the variables in the file
            let content = fs::read_to_string(&dest_path_str)?;
            let mut new_content = content.clone();
            for (var_name, var_value) in &self.source_variable_values {
                let var_name = format!(
                    "{}{}{}",
                    app_config::TEMPLATE_VAR_PREFIX,
                    var_name,
                    app_config::TEMPLATE_VAR_SUFFIX
                );
                new_content = new_content.replace(&var_name, var_value);
            }
            // Write the new content
            fs::write(&dest_path_str, new_content)?;
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
