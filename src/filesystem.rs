use std::fs;
use std::process::exit;
use crate::constants::{COULD_NOT_CREATE_DOT_BITA_FOLDER_MSG, COULD_NOT_FIND_HOME_DIR_MSG};

pub(crate) fn get_dot_bita_folder_path() -> String {
    match home::home_dir() {
        Some(path) => {
            return format!("{}/.bita", path.as_path().display().to_string());
        },
        None => {
            println!("{}", COULD_NOT_FIND_HOME_DIR_MSG);
            exit(1);
        },
    }
}

pub(crate) fn create_dot_bita_folder_if_does_not_exists() {
    match fs::create_dir_all(get_dot_bita_folder_path()) {
        Ok(_) => {
            println!("$HOME/.bita folder created");
        }
        Err(_) => {
            println!("{}", COULD_NOT_CREATE_DOT_BITA_FOLDER_MSG);
            exit(1);
        }
    };
}