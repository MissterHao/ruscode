use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("")]
    CreateDatabaseFailed,

    #[error("Permission denied.")]
    CreateDatabasePermissionDenied,
}
