use std::path::Path;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::constants;

pub(crate) fn register_new_event(message: String) {
    write_to_bita_db_sqlite(message);
}

fn init_bita_db_if_does_not_exists() {
    if Path::new(&get_bita_db_path()).exists() {
        return;
    }
    println!("{}", constants::BITA_DB_SQLITE_CREATION_MSG);
    let connection = sqlite::open(get_bita_db_path()).unwrap();
    let query = "
    CREATE TABLE events (
        id TEXT,
        message TEXT,
        created_at NUMERIC
        );
    ";
    connection.execute(query).unwrap();
}

fn write_to_bita_db_sqlite(message: String) {
    init_bita_db_if_does_not_exists();
    let connection = sqlite::open(get_bita_db_path()).unwrap();
    let query = "
    INSERT INTO events (id, message, created_at) VALUES (:id, :message, :created_at);
    ";
    let id = Uuid::new_v4().to_string();
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((":id", id.trim())).unwrap();
    statement.bind((":message", message.trim())).unwrap();
    statement.bind((":created_at", get_now_timestamp())).unwrap();
    statement.next().unwrap();
    println!("{} {}", constants::SUCCESSFULLY_SAVED_NEW_EVENT_MSG, id);
}

fn get_now_timestamp() -> i64 {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    return now.timestamp();
}

fn get_bita_db_path() -> String {
    match home::home_dir() {
        Some(path) => {
            return format!("{}/.bita/bita.db", path.as_path().display().to_string());
        },
        None => {
            println!("Impossible to get your home dir!");
            panic!();
        },
    }
}