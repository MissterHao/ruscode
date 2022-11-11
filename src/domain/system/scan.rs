use std::io::Error;
use std::str::FromStr;
use std::{fs, str};
extern crate glob;
use crate::common::system::SystemPaths;
use crate::domain::entity::workspace::Workspace;
use crate::domain::value_object::WorkspaceJson;
use glob::glob;

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
        Err(_) => {
            // println!("Error!! => {}", path);
            None
        }
    }
}

pub fn scan_workspaces_path() {
    let current_workspaces_list = scan_vscode_workspacestorage_from_system();

    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};
    use std::thread;

    // Spawn a thread channel between app and new thread
    let (tx, rx): (Sender<Option<Workspace>>, Receiver<Option<Workspace>>) = mpsc::channel();

    let mut children = Vec::new();

    for json_path in current_workspaces_list.unwrap() {
        let thread_tx = tx.clone();

        let child = thread::spawn(move || {
            let data = extract_json_file(json_path.as_str());

            match data {
                Some(val) => {
                    thread_tx.send(Some(Workspace::from(val)));
                }
                None => {
                    thread_tx.send(None);
                }
            }
        });

        children.push(child);
    }

    // Show the order in which the messages were sent

    let mut result = Vec::new();

    for _ in 0..children.len() {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        result.push(rx.recv());
    }

    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    for r in result {
        if let Some(val) = r.unwrap() {
            println!("{:?}", val.location_type);
        }
    }
}
