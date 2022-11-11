use std::io::Error;
use std::str::FromStr;
use std::{fs, str};
extern crate glob;
use crate::common::system::SystemPaths;
use crate::domain::entity::workspace::Workspace;
use crate::domain::value_object::WorkspaceJson;
use glob::glob;
use std::thread;

fn scan_vscode_workspacestorage_from_system() -> Result<Vec<String>, Error> {
    let home = SystemPaths::home_dir();
    let tasks = glob(SystemPaths::vscode_workspace_storage_path().as_str())
        .expect("Fali to read glob pattern")
        .into_iter()
        .map(|entry| String::from_str(entry.unwrap().to_str().unwrap()).unwrap())
        .collect::<Vec<String>>();

    Ok(tasks)
}

fn extract_json_file(path: &str) -> Option<WorkspaceJson> {
    let raw_json = fs::read_to_string(path).expect("Cannot read workspace from json");
    match serde_json::from_str(raw_json.as_str()) {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}

pub fn scan_workspaces_path() -> Vec<Workspace> {
    use crossbeam_channel::{bounded, Sender};

    // Get all vscode workspace json files path
    let current_workspaces_list: Result<Vec<String>, Error> =
        scan_vscode_workspacestorage_from_system();

    let (s, r) = bounded(0);
    thread::spawn(move || {
        for json_path in current_workspaces_list.unwrap() {
            let data = extract_json_file(json_path.as_str());

            match data {
                Some(val) => {
                    s.send(Some(Workspace::from(val)))
                        .expect("Fail to send Workspace struct to main receive channel.");
                }
                None => {
                    // s.send(None);
                }
            }
        }
    });

    let mut workapaces: Vec<Workspace> = Vec::new();

    // Print the first 20 Fibonacci numbers.
    for option_val in r.iter() {
        if let Some(val) = option_val {
            workapaces.push(val);
        }
    }

    workapaces
}
