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
use crate::app_config::{CREATOR_ENV_VAR, DEFAULT_CREATOR_ENV_VAR_VALUE};
use directories::UserDirs;
use std::env;

pub fn get_current_working_directory() -> String {
    env::current_dir()
        .expect("CWD is not accessible!")
        .to_str()
        .expect("Cannot covert to String")
        .to_string()
}

pub fn get_storage_path() -> String {
    // try to get the storage path from the environment variable
    let creator_env_var = env::var(CREATOR_ENV_VAR);

    // If the environment variable is set, use if not use the default value, create a dir if not created and use
    let storage;
    if creator_env_var.is_ok() {
        storage = creator_env_var.unwrap();
        return storage;
    } else {
        storage = DEFAULT_CREATOR_ENV_VAR_VALUE.to_string();
        // create the directory if it does not exist
        if !std::path::Path::new(&storage).exists() {
            std::fs::create_dir_all(&storage).expect("Cannot create storage directory");
        }
    }

    // Unfold ~ to the user's home directory
    if storage.starts_with("~") {
        if let Some(user_dirs) = UserDirs::new() {
            let home_dir = user_dirs.home_dir().to_str().unwrap();
            let unfolded = storage.replacen("~", home_dir, 1);

            // create the directory if it does not exist
            if !std::path::Path::new(&unfolded).exists() {
                std::fs::create_dir_all(&unfolded)
                    .expect("Cannot create storage directory");
                println!("Created storage directory: {}", unfolded);
            }
            return unfolded;
        }
    }
    storage
}
