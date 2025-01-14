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
use crate::creator::Creator;
use directories::UserDirs;
use std::env;

pub fn get_current_working_directory() -> String {
    env::current_dir()
        .expect("CWD is not accessible!")
        .to_str()
        .expect("Cannot covert to String")
        .to_string()
}

pub fn unfold_path(path: &str) -> String {
    if path.starts_with("~") {
        if let Some(user_dirs) = UserDirs::new() {
            let home_dir = user_dirs.home_dir().to_str().unwrap();
            return path.replacen("~", home_dir, 1);
        }
    }
    path.to_string()
}

pub fn get_storage_path() -> String {
    // try to get the storage path from the environment variable
    let creator_env_var = env::var(Creator::CREATOR_ENV_VAR);

    // If the environment variable is set, use if not use the default value, create a dir if not created and use
    let storage = creator_env_var.unwrap_or_else(|_| Creator::DEFAULT_CREATOR_ENV_VAR_VALUE.to_string());

    // Unfold ~ to the user's home directory
    unfold_path(&storage)
}
