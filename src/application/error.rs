use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum ApplicationError {
    #[error("Database file can't create at `{0}`")]
    InitializeDatabaseFailed(String),

    #[error("Workspaces can't be scan at `{0}`")]
    InitializeWorkspaceSacnFailed(String),
}
