use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum ApplicationInitError {
    #[error("")]
    CannotCreateDatabaseFolder,
}
