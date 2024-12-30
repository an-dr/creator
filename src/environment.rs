use directories::UserDirs;
use std::env;

use crate::app_config::{CREATOR_ENV_VAR, DEFAULT_CREATOR_ENV_VAR_VALUE};

pub fn get_current_working_directory() -> String {
    env::current_dir()
        .expect("CWD is not accessible!")
        .to_str()
        .expect("Cannot covert to String")
        .to_string()
}

pub fn get_storage_path() -> String {
    let storage = std::env::var(CREATOR_ENV_VAR)
        .unwrap_or_else(|_| DEFAULT_CREATOR_ENV_VAR_VALUE.to_string());

    // Unfold ~ to the user's home directory
    if storage.starts_with("~") {
        if let Some(user_dirs) = UserDirs::new() {
            let home_dir = user_dirs.home_dir().to_str().unwrap();
            return storage.replacen("~", home_dir, 1);
        }
    }
    storage
}
