use std::fs;

use super::error::ApplicationInitError;
use crate::common::system::SystemPaths;

pub fn init_application_folders() -> Result<(), ApplicationInitError> {
    let database_path = SystemPaths::database_folder();
    fs::create_dir_all(database_path);

    Ok(())
}
