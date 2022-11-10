use std::io::Error;
use std::str;
extern crate glob;
use crate::common::system::SystemPaths;
use futures;
use futures::stream::FuturesUnordered;
use glob::glob;
use std::path::PathBuf;
use tokio::task::JoinError;

pub async fn scan_vscode_workspacestorage_from_system(
) -> Result<Vec<Result<PathBuf, JoinError>>, Error> {
    let home = SystemPaths::home_dir();
    let tasks = glob(
        format!(
            "{}/AppData/Roaming/Code/User/workspaceStorage/**/*.json",
            home,
        )
        .as_str(),
    )
    .expect("Fali to read glob pattern")
    .into_iter()
    .map(|entry| {
        tokio::spawn(async move {
            // sleep(Duration::from_secs(1)).await; // simulate some work
            entry.unwrap()
        })
    })
    .collect::<FuturesUnordered<_>>();

    let result = futures::future::join_all(tasks).await;
    println!("{:?}", result);   

    Ok(result)
}
