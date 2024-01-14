pub(crate) const CREATE_ENTRIES_TABLE: &str = "
    CREATE TABLE entries (
        id TEXT,
        message TEXT,
        created_at TEXT
        );
    ";

pub(crate) const INSERT_ENTRY: &str = "
    INSERT INTO entries (id, message, created_at) VALUES (:id, :message, :created_at);
    ";

pub(crate) const DELETE_ENTRY: &str = "DELETE FROM entries WHERE id = :id";

pub(crate) const SELECT_ONE_ENTRY: &str = "SELECT * FROM entries WHERE id = :id";

pub(crate) const SELECT_ALL_ENTRIES: &str = "SELECT * FROM entries ORDER BY created_at DESC";
