mod constants;
mod help;
mod register_new_entry;
mod version;
mod delete_entry;
mod sqlite;
mod timestamps;
mod view_entries;
mod queries;
mod entries_repository;

use std::env;
use std::process::exit;
use colored::Colorize;
use crate::view_entries::{get_entries, get_entry};

fn main() {
    match env::args().nth(1) {
        Some(message) => {
            exec(message);
        }
        None => {
            help::help();
        }
    }
}

fn exec(command: String) {
    match command.trim() {
        "n" | "new" => {
            register_new_entry::new_entry(env::args().nth(2).unwrap());
        }
        "d" | "delete" => {
            delete_entry::delete_entry(env::args().nth(2).unwrap());
        }
        "a" | "all" => {
            get_entries();
        }
        "g" | "get" => {
            get_entry(env::args().nth(2).unwrap());
        }
        "c" | "config" => {
            println!("Not implemented yet...");
        }
        "v" | "version" => {
            version::version();
        }
        "h" | "help" => {
            help::help();
        }
        _ => {
            println!("Invalid command! Check {} for more info", "bita help".bold());
            exit(1);
        }
    }
    exit(0);
}