use std::process::exit;
use colored::Colorize;
use sqlite::{State, Statement};
use crate::constants;
use crate::constants::ENTRY_ID_SIZE;
use crate::queries::{SELECT_ALL_ENTRIES, SELECT_ONE_ENTRY};
use crate::sqlite::sqlite_conn;

pub(crate) fn get_entry(id: String) {
    if id.chars().count() != ENTRY_ID_SIZE {
        println!("{} {}", constants::ENTRY_NOT_FOUND_MSG, id);
        exit(1);
    }
    let connection = sqlite_conn().unwrap();
    let mut statement = connection.prepare(SELECT_ONE_ENTRY).unwrap();
    statement.bind((":id", id.trim())).unwrap();
    while let Ok(State::Row) = statement.next() {
        print_entry(&statement);
    }
    if statement.iter().count() == 0 {
        println!("{} {}", constants::ENTRY_NOT_FOUND_MSG, id);
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