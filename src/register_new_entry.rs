use uuid::Uuid;
use crate::constants;
use crate::sqlite::{init_sqlite_db_if_does_not_exists, sqlite_conn};
use crate::timestamps::get_now_timestamp;

pub(crate) fn new_entry(message: String) {
    init_sqlite_db_if_does_not_exists();
    let connection = sqlite_conn().unwrap();
    let query = "
    INSERT INTO entries (id, message, created_at) VALUES (:id, :message, :created_at);
    ";
    let id = Uuid::new_v4().to_string();
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((":id", id.trim())).unwrap();
    statement.bind((":message", message.trim())).unwrap();
    statement.bind((":created_at", get_now_timestamp())).unwrap();
    statement.next().unwrap();
    println!("{} {}", constants::SUCCESSFULLY_SAVED_NEW_ENTRY_MSG, id);
}
