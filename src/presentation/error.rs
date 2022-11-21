use thiserror::Error;

use crate::domain::system;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum UIError {
    #[error("Database file can't create at `{0}`")]
    DisplayWorkspaceFailed(#[from] system::error::SystemError),
}
