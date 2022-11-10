use std::io::Error;
use std::str::FromStr;
use std::{str, task};
extern crate glob;
use crate::common::system::SystemPaths;
use crate::domain::entity::workspace::Workspace;
use futures;
use futures::stream::FuturesUnordered;
use glob::glob;
use std::path::PathBuf;
use tokio::task::JoinError;

pub fn scan_vscode_workspacestorage_from_system() -> Result<Vec<String>, JoinError> {
    let home = SystemPaths::home_dir();
    let tasks = glob(SystemPaths::vscode_workspace_storage_path().as_str())
        .expect("Fali to read glob pattern")
        .into_iter()
        .map(|entry| String::from_str(entry.unwrap().to_str().unwrap()).unwrap())
        .collect::<Vec<String>>();

    Ok(tasks)
}
