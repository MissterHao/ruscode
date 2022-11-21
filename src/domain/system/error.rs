use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum ApplicationInitError {
    #[error("")]
    CannotCreateDatabaseFolder,
    #[error("")]
    OpenWorkspaceFolderFailed,
}

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum SystemError {
    #[error("Couldn't access local directory")]
    OpenWorkspaceFolderFailed,
}
