use std::fs;

use super::error::ApplicationInitError;
use crate::common::system::SystemPaths;

pub fn init_application_folders(path: Option<String>) -> Result<(), ApplicationInitError> {
    let create_path: String = match path {
        Some(val) => val,
        None => SystemPaths::database_folder(),
    };
    fs::create_dir_all(create_path).expect("Cannot create application folders.");
    Ok(())
}

#[cfg(test)]
mod test_system_init {
    use std::fs;

    use super::init_application_folders;

    #[test]
    fn test_fake_application_folder_creation() {
        let fake_dir_path = "fake/dir/folder";
        let fake_dir_path_root = "fake";

        match init_application_folders(Some(String::from(fake_dir_path))) {
            Ok(_) => {
                fs::remove_dir_all(fake_dir_path_root).expect("Remove fake dirs failed.");
            }
            Err(_) => {
                panic!("Cannot create fake application folder.");
            }
        };
    }
}
