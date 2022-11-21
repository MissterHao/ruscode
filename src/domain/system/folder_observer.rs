use chrono::{format, DateTime, TimeZone, Utc};

use crate::domain::entity::workspace::Workspace;
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use super::error::SystemError;

pub fn last_modified(workspace: &Workspace) -> Result<String, SystemError> {
    // let mut entries: Vec<fs::DirEntry> = fs::read_dir(workspace.strip_decode_path())
    //     .expect("Couldn't access local directory")
    //     .flatten() // Remove failed
    //     .collect();
    let result: Result<Vec<fs::DirEntry>, _> = match fs::read_dir(workspace.strip_decode_path()) {
        Ok(val) => Ok(val.flatten().collect()),
        Err(_) => return Err(SystemError::OpenWorkspaceFolderFailed),
    };

    let mut entries = result?;
    entries.sort_by_cached_key(|f| f.metadata().unwrap().modified().unwrap());

    let last_modified_secs = entries[0]
        .metadata()
        .unwrap()
        .modified()
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Ok(Utc
        .timestamp_opt(last_modified_secs as i64, 0)
        .unwrap()
        .format("%Y/%m/%d")
        .to_string())
}

#[cfg(test)]
mod test_folder_observer {
    #[test]
    fn test_last_modified() {
        // let sys_time = last_modified();
        // let last_modified_secs = sys_time.duration_since(UNIX_EPOCH).unwrap().as_secs();

        // println!(
        //     "{:?}",
        //     Utc.timestamp_opt(last_modified_secs as i64, 0)
        //         .unwrap()
        //         .format("%Y/%m/%d")
        //         .to_string()
        // );

        // println!("{:?}", last_modified_secs);
    }
}
