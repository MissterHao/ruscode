use crate::domain::value_object::WorkspaceJson;
use rusqlite::Row;
use urlencoding::decode;

use std::{
    hash::{Hash, Hasher},
    path::Path,
};

/// Workspace Location enumerate
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceLocation {
    NotRecognize,
    Local,
    Remote,
}

/// Implement default associate function for Workspace Location enumerate
impl WorkspaceLocation {
    fn default() -> Self {
        WorkspaceLocation::NotRecognize
    }
}

impl From<&str> for WorkspaceLocation {
    fn from(path: &str) -> Self {
        if path.starts_with("file://") {
            WorkspaceLocation::Local
        } else if path.starts_with("vscode-remote://") {
            WorkspaceLocation::Remote
        } else {
            // Default
            WorkspaceLocation::NotRecognize
        }
    }
}

/// Workspace database transfer object
#[derive(Debug, Clone, Eq)]
pub struct Workspace {
    pub path: String,
    pub decode_path: String,
    pub location_type: WorkspaceLocation,
    pub title: String,
}

impl Hash for Workspace {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

/// Implement default associate function for Workspace Location enumerate
impl Workspace {
    pub fn new() -> Self {
        Workspace {
            path: String::new(),
            decode_path: String::new(),
            location_type: WorkspaceLocation::default(),
            title: String::new(),
        }
    }

    pub fn from_dbrow(row: &Row) -> Self {
        let raw_path: String = row.get(0).expect("msg");
        Workspace {
            path: raw_path.clone(),
            decode_path: decode(raw_path.as_str()).expect("UTF-8").to_string(),
            location_type: raw_path.as_str().into(),
            title: Path::new(&raw_path)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        }
    }
}

impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl From<WorkspaceJson> for Workspace {
    fn from(_wj: WorkspaceJson) -> Self {
        let decode_folder_path = decode(_wj.folder.as_str()).expect("UTF-8").to_string();
        let location = WorkspaceLocation::from(_wj.folder.as_str());
        let workspace_path_obj = Path::new(decode_folder_path.as_str());

        Workspace {
            path: _wj.folder,
            decode_path: decode_folder_path.clone(),
            location_type: location,
            title: workspace_path_obj
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        }
    }
}

impl From<&str> for Workspace {
    fn from(raw_path: &str) -> Self {
        let decode_folder_path = decode(raw_path).expect("UTF-8").to_string();

        let location = WorkspaceLocation::from(raw_path);
        let workspace_path_obj = Path::new(raw_path);

        Workspace {
            path: raw_path.to_string(),
            decode_path: decode_folder_path,
            location_type: location,
            title: workspace_path_obj
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        }
    }
}
