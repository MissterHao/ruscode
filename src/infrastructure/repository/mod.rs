pub mod error;
pub mod workspace_repository;

use self::error::DatabaseError;
use rusqlite::{Connection, Result};

const SQL: &str = r#"


CREATE TABLE workspaces (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    title TEXT NOT NULL,
    path TEXT NOT NULL,
    type TEXT NOT NULL
);

CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    name TEXT NOT NULL
);

CREATE TABLE tags_workspaces (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    workspaces_id INTEGER,
    tags_id INTEGER,
    FOREIGN KEY(workspaces_id) REFERENCES workspaces(id),
    FOREIGN KEY(tags_id) REFERENCES tags(id)
);

CREATE INDEX idx_tags ON tags_workspaces(tags_id);
"#;

/// Create database file or pass if database is already existed.
pub fn create_database(path: &str) -> Result<bool, DatabaseError> {
    let db_connection: Connection = match Connection::open(path) {
        Ok(con) => con,
        Err(err) => {
            println!("Open error");
            Connection::open(path)?
        }
    };

    match db_connection.execute(SQL, ()) {
        Ok(val) => {
            // successfully create table
        }
        Err(e) => {
            // table already existed
        }
    }

    Ok(true)
}

pub fn get_db_connection(path: &str) -> Result<Connection, DatabaseError> {
    let db_connection: Connection = match Connection::open(path) {
        Ok(con) => con,
        Err(err) => Connection::open(path)?,
    };

    Ok(db_connection)
}
