use crate::common::text::strip_trailing_newline;
use std::process::{Command, Output};
use std::str;

pub struct SystemPaths {}

impl SystemPaths {
    #[cfg(target_os = "windows")]
    fn user_home_dir() -> Output {
        Command::new("cmd")
            .args(["/C", "echo %userprofile%"])
            .output()
            .expect("failed to execute process")
    }

    #[cfg(target_os = "linux")]
    fn user_home_dir() -> Output {
        Command::new("sh")
            .arg("-c")
            .arg("getent passwd \"$USER\" | cut -d: -f6 ")
            .output()
            .expect("failed to execute process")
    }

    #[cfg(target_os = "windows")]
    pub fn home_dir() -> String {
        let output = SystemPaths::user_home_dir();

        let home = strip_trailing_newline(str::from_utf8(&output.stdout).unwrap());

        home.to_string().replace("\\\\", "\\")
    }

    #[cfg(target_os = "linux")]
    pub fn home_dir() -> String {
        let output = SystemPaths::user_home_dir();

        let home = strip_trailing_newline(str::from_utf8(&output.stdout).unwrap());

        home.to_string().replace("\\\\", "\\")
    }

    #[cfg(target_os = "windows")]
    pub fn vscode_workspace_storage_path() -> String {
        let home = SystemPaths::home_dir();
        format!(
            "{}/AppData/Roaming/Code/User/workspaceStorage/**/workspace.json",
            home,
        )
    }

    #[cfg(target_os = "linux")]
    pub fn vscode_workspace_storage_path() -> String {
        let home = SystemPaths::home_dir();
        format!(
            "{}/.config/Code/User/workspaceStorage/**/workspace.json",
            home
        )
    }

    // Application local folder
    /// Application local folder for Windows Operation System
    #[cfg(target_os = "windows")]
    pub fn application_data_folder() -> String {
        let home = SystemPaths::home_dir();
        format!("{}/AppData/Local/ruscode", home)
    }

    /// Application local folder for not Windows Operation System
    #[cfg(target_os = "linux")]
    pub fn application_data_folder() -> String {
        String::from("~/.ruscode")
    }

    /// Database path
    pub fn database_folder() -> String {
        format!("{}/database", SystemPaths::application_data_folder())
    }

    /// Database path
    pub fn database() -> String {
        if cfg!(test) {
            format!("{}/data.db", SystemPaths::database_folder())
        } else {
            format!("{}/data-test.db", SystemPaths::database_folder())
        }
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

    #[cfg(target_os = "windows")]
    #[test]
    fn test_systempaths_on_windows() {
        SystemPaths::user_home_dir();
    }
    #[cfg(target_os = "windows")]
    #[test]
    fn test_systempaths_on_windows_re_format() {
        let output = SystemPaths::user_home_dir();
        let path = str::from_utf8(&output.stdout).unwrap();
        let re = regex::Regex::new(r".*\\Users\\.*").unwrap();
        assert!(re.is_match(path));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_systempaths_on_linux() {
        SystemPaths::user_home_dir();
    }
    #[cfg(target_os = "linux")]
    #[test]
    fn test_systempaths_on_linux_re_format() {
        let output = SystemPaths::user_home_dir();
        let path = str::from_utf8(&output.stdout).unwrap();
        let re = regex::Regex::new(r"/home/.*").unwrap();
        assert!(re.is_match(path));
    }

    #[test]
    fn test_systempaths_get_home_dir() {
        let home_dir = SystemPaths::home_dir();
        assert_eq!(type_of(&home_dir), type_of(&String::new()));

        let re = regex::Regex::new(r"\\\\").unwrap();
        assert_eq!(re.find_iter(home_dir.as_str()).count(), 0);
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_vscode_workspace_storage_path_on_windows() {
        let path = SystemPaths::vscode_workspace_storage_path();
        let re =
            regex::Regex::new(".*/AppData/Roaming/Code/User/workspaceStorage/.*/workspace.json")
                .unwrap();

        assert!(re.is_match(path.as_str()));
    }
    #[cfg(target_os = "linux")]
    #[test]
    fn test_vscode_workspace_storage_path_on_linux() {
        SystemPaths::vscode_workspace_storage_path();
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_application_data_folder_path_on_windows() {
        let path = SystemPaths::application_data_folder();
        let re = regex::Regex::new(".*/AppData/Local/ruscode").unwrap();
        assert!(re.is_match(path.as_str()));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_application_data_folder_path_on_linux() {
        let path = SystemPaths::application_data_folder();
        let re = regex::Regex::new("~/.ruscode").unwrap();
        assert!(re.is_match(path.as_str()));
    }

    #[test]
    fn test_database_folder() {
        let path = SystemPaths::application_data_folder();
        assert_eq!(format!("{}/database", path), SystemPaths::database_folder());
    }

    #[test]
    fn test_database_file_path() {
        let path = SystemPaths::database_folder();
        assert_eq!(format!("{}/data.db", path), SystemPaths::database());
    }
}
