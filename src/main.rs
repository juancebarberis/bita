mod constants;
mod help;
mod register_new_event;
mod version;

use std::env;

fn main() {
    match env::args().nth(1) {
        Some(message) => {
            register_new_event::register_new_event(message);
        }
        None => {
            help::help();
        }
    }
}