use std::path::Path;
use sqlite::Connection;
use crate::constants;

pub(crate) fn get_sqlite_db_path() -> String {
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

pub(crate) fn sqlite_conn() -> sqlite::Result<Connection> {
    return sqlite::open(get_sqlite_db_path());
}

pub(crate) fn init_sqlite_db_if_does_not_exists() {
    if Path::new(&get_sqlite_db_path()).exists() {
        return;
    }
    println!("{}", constants::BITA_DB_SQLITE_CREATION_MSG);
    let connection = sqlite_conn().unwrap();
    let query = "
    CREATE TABLE events (
        id TEXT,
        message TEXT,
        created_at NUMERIC
        );
    ";
    connection.execute(query).unwrap();
}