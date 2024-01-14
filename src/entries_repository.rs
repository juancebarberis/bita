use std::fmt::Error;
use sqlite::State;
use crate::queries::{DELETE_ENTRY, SELECT_LATEST_ENTRY};
use crate::sqlite::sqlite_conn;

pub(crate) fn get_latest_entry_id() -> Result<String, Error> {
    let connection = sqlite_conn().unwrap();
    let mut statement = connection.prepare(SELECT_LATEST_ENTRY).unwrap();
    while let Ok(State::Row) = statement.next() {
        return Ok(statement.read::<String, _>("id").unwrap());
    }
    return Err(Error::default());
}

pub(crate) fn delete_by_id(id: String) -> usize {
    let connection = sqlite_conn().unwrap();
    let mut statement = connection.prepare(DELETE_ENTRY).unwrap();
    statement.bind((":id", id.trim())).unwrap();
    statement.next().unwrap();
    return connection.total_change_count();
}