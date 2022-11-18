use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum DatabaseError {
    #[error("Can't open database file")]
    CannotOpenDatabaseFile(#[from] rusqlite::Error),

    #[error("")]
    CreateDatabaseFailed,

    #[error("Permission denied.")]
    CreateDatabasePermissionDenied,
}
