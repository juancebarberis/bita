use std::process::exit;
use colored::Colorize;
use sqlite::{State, Statement};
use crate::constants;
use crate::queries::{SELECT_ALL_ENTRIES, SELECT_ONE_ENTRY};
use crate::sqlite::sqlite_conn;

pub(crate) fn get_entry(id: String) {
    let connection = sqlite_conn().unwrap();
    let mut statement = connection.prepare(SELECT_ONE_ENTRY).unwrap();
    statement.bind((":id", id.trim())).unwrap();
    while let Ok(State::Row) = statement.next() {
        print_entry(&statement);
    }
    if statement.iter().count() == 0 {
        println!("{}", constants::ENTRY_NOT_FOUND_MSG);
        exit(1);
    }
}

pub(crate) fn get_entries() {
    let connection = sqlite_conn().unwrap();
    let mut statement = connection.prepare(SELECT_ALL_ENTRIES).unwrap();
    while let Ok(State::Row) = statement.next() {
        print_entry(&statement);
        println!();
    }
}

fn print_entry(statement: &Statement) {
    println!(
        "{} (Timestamp: {})",
        statement.read::<String, _>("id").unwrap().green(),
        statement.read::<String, _>("created_at").unwrap()
    );
    println!("{}", statement.read::<String, _>("message").unwrap());
}