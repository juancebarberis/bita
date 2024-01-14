mod constants;
mod help;
mod register_new_entry;
mod version;
mod delete_entry;
mod sqlite;
mod timestamps;

use std::env;

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
            register_new_entry::new_entry(command);
        }
        "d" | "delete" => {
            delete_entry::delete_entry("Hola".parse().unwrap());
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
        _ => {}
    }
}