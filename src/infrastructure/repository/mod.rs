pub mod error;
pub mod macros;
pub mod workspace_repository;

use self::error::DatabaseError;
use rusqlite::{Connection, Result};

const SQL: &str = r#"


CREATE TABLE workspaces (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    path TEXT NOT NULL
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
        Err(_err) => {
            println!("Open error");
            Connection::open(path)?
        }
    };

    match db_connection.execute(SQL, ()) {
        Ok(_val) => {
            // successfully create table
        }
        Err(_e) => {
            // table already existed
        }
    }

    Ok(true)
}

pub fn get_db_connection(path: &str) -> Result<Connection, DatabaseError> {
    let db_connection: Connection = match Connection::open(path) {
        Ok(con) => con,
        Err(_err) => Connection::open(path)?,
    };

    Ok(db_connection)
}

#[cfg(test)]
mod test_database {
    use std::fs;

    use super::{create_database, get_db_connection};

    #[test]
    fn test_database_should_create_succesfully() {
        let testing_database_path = "./test-database.sqlite3";
        if create_database(testing_database_path).is_ok() {
            fs::remove_file(testing_database_path).unwrap();
        } else {
            panic!("Database create failed!")
        }
    }

    #[test]
    fn test_database_connection_should_succesfully() {
        let testing_database_path = "./test-database-conn.sqlite3";
        if create_database(testing_database_path).is_ok() {
            get_db_connection(testing_database_path).expect("Cannot get database connection!");
            fs::remove_file(testing_database_path).unwrap();
        } else {
            panic!("Database create failed!")
        }
    }
}
