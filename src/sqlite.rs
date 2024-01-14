use std::path::Path;
use std::process::exit;
use sqlite::Connection;
use crate::constants;
use crate::queries::CREATE_ENTRIES_TABLE;

pub(crate) fn get_sqlite_db_path() -> String {
    match home::home_dir() {
        Some(path) => {
            return format!("{}/.bita/bita.db", path.as_path().display().to_string());
        },
        None => {
            println!("{}", constants::COULD_NOT_FIND_HOME_DIR_MSG);
            exit(1);
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
    connection.execute(CREATE_ENTRIES_TABLE).unwrap();
}