use std::process::exit;
use crate::constants;
use crate::constants::ENTRY_ID_SIZE;
use crate::sqlite::sqlite_conn;

pub(crate) fn delete_entry(id: String) {
    if id.chars().count() != ENTRY_ID_SIZE {
        println!("Invalid id format");
        exit(1);
    }
    let connection = sqlite_conn().unwrap();
    let query = "DELETE FROM entries WHERE id = :id";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((":id", id.trim())).unwrap();
    statement.next().unwrap();
    let rows_affected = connection.total_change_count();

    if rows_affected > 0 {
        println!("{} {}", constants::SUCCESSFULLY_DELETED_NEW_ENTRY_MSG, id);
    } else {
        println!("{} {}", constants::ENTRY_NOT_FOUND_MSG, id);
        exit(1);
    }
}
