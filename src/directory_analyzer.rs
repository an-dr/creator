// *************************************************************************
//
// Copyright (c) 2025 Andrei Gramakov. All rights reserved.
//
// This file is licensed under the terms of the MIT license.
// For a copy, see: https://opensource.org/licenses/MIT
//
// site:    https://agramakov.me
// e-mail:  mail@agramakov.me
//
// *************************************************************************

use regex::Regex;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use crate::app_config;

pub struct DirectoryAnalyzer {
    path: PathBuf,
}

impl DirectoryAnalyzer {
    pub fn new<P: AsRef<Path>>(path: P) -> DirectoryAnalyzer {
        DirectoryAnalyzer {
            path: PathBuf::from(path.as_ref()),
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
        let pattern = format!(
            "{}(.*?){}",
            app_config::TEMPLATE_VAR_PREFIX,
            app_config::TEMPLATE_VAR_SUFFIX
        );
        let re = Regex::new(&pattern).expect("Matching pattern must be accepted");
        for caps in re.captures_iter(text) {
            if let Some(var_name) = caps.get(1) {
                vars_to_append.insert(var_name.as_str().to_string());
                // println!("Found: {}", var_name.as_str());
            }
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
