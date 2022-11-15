use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {

    #[error("Can't open database file")]
    CannotOpenDatabaseFile(#[from] rusqlite::Error),

    #[error("")]
    CreateDatabaseFailed,

    #[error("Permission denied.")]
    CreateDatabasePermissionDenied,
}
