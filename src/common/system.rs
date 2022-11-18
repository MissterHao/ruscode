use crate::common::text::strip_trailing_newline;
use std::process::Command;
use std::str;

pub struct SystemPaths {}

impl SystemPaths {
    pub fn home_dir() -> String {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "echo %userprofile%"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("getent passwd \"$USER\" | cut -d: -f6 ")
                .output()
                .expect("failed to execute process")
        };

        let home = strip_trailing_newline(str::from_utf8(&output.stdout).unwrap());

        home.to_string().replace("\\\\", "\\")
    }

    pub fn vscode_workspace_storage_path() -> String {
        let home = SystemPaths::home_dir();
        if cfg!(target_os = "windows") {
            format!(
                "{}/AppData/Roaming/Code/User/workspaceStorage/**/workspace.json",
                home,
            )
        } else {
            todo!()
        }
    }

    // Application local folder
    /// Application local folder for Windows Operation System
    #[cfg(target_os = "windows")]
    pub fn application_data_folder() -> String {
        let home = SystemPaths::home_dir();
        format!("{}/AppData/Local/ruscode", home)
    }

    /// Application local folder for not Windows Operation System
    #[cfg(not(target_os = "windows"))]
    pub fn application_data_folder() -> String {
        String::new("/var/lib/ruscode")
    }

    /// Database path
    pub fn database_folder() -> String {
        format!("{}/database", SystemPaths::application_data_folder())
    }

    /// Database path
    pub fn database() -> String {
        format!("{}/data.db", SystemPaths::database_folder())
    }
}

#[cfg(test)]
mod test_system {

    use super::*;
    use std::any::type_name;

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }

    #[test]
    fn test_systempaths_homedir_should_work_without_panic() {
        assert_eq!(type_of(SystemPaths::home_dir()), type_of(String::new()));
    }
}
