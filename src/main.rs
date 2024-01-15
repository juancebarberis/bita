mod constants;
mod help;
mod new_entry;
mod version;
mod delete_entry;
mod sqlite;
mod timestamps;
mod view_entries;
mod queries;
mod entries_repository;
mod filesystem;

use std::env;
use std::process::exit;
use colored::Colorize;
use crate::delete_entry::delete_entry;
use crate::help::help;
use crate::new_entry::new_entry;
use crate::version::version;
use crate::view_entries::{get_entries, get_entry};

fn main() {
    match env::args().nth(1) {
        Some(message) => {
            exec(message);
        }
        None => {
            help();
        }
    }
}

fn exec(command: String) {
    match command.trim() {
        "n" | "new" => {
            new_entry(
                get_param_or_exit(2, "message")
            );
        }
        "d" | "delete" => {
            delete_entry(
                get_param_or_exit(2, "ID, --latest|-l")
            );
        }
        "a" | "all" => {
            get_entries(
                env::args().nth(2)
            );
        }
        "g" | "get" => {
            get_entry(
                get_param_or_exit(2, "ID, --latest|-l")
            );
        }
        "c" | "config" => {
            println!("Not implemented yet...");
        }
        "v" | "version" => {
            version();
        }
        "h" | "help" => {
            help();
        }
        _ => {
            println!("Invalid command");
            println!("Check {} for help", "bita help".bold());
            exit(1);
        }
    }
    exit(0);
}

fn get_param_or_exit(nth: usize, param_name: &str) -> String {
    let param = env::args().nth(nth);
    if param.is_none() {
        println!("Param {} required", param_name.bright_cyan());
        println!("Check {} for help", "bita help".bold());
        exit(1);
    }
    return param.unwrap();
}