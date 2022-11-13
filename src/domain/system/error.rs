use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationInitError {
    #[error("")]
    CannotCreateDatabaseFolder,
}
