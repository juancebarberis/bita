use std::path::Path;
use sqlite::Connection;
use crate::constants;
use crate::filesystem::{create_dot_bita_folder_if_does_not_exists, get_dot_bita_folder_path};
use crate::queries::CREATE_ENTRIES_TABLE;

pub(crate) fn get_sqlite_db_path() -> String {
    return format!("{}/bita.db", get_dot_bita_folder_path());
}

pub(crate) fn sqlite_conn() -> sqlite::Result<Connection> {
    return sqlite::open(get_sqlite_db_path());
}

pub(crate) fn init_sqlite_db_if_does_not_exists() {
    if Path::new(&get_sqlite_db_path()).exists() {
        return;
    }
    create_dot_bita_folder_if_does_not_exists();
    println!("{}", constants::BITA_DB_SQLITE_CREATION_MSG);
    let connection = sqlite_conn().unwrap();
    connection.execute(CREATE_ENTRIES_TABLE).unwrap();
}