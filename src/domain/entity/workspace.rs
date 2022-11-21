use crate::domain::value_object::WorkspaceJson;
use regex::Regex;
use rusqlite::Row;
use urlencoding::decode;

use std::hash::{Hash, Hasher};

/// Workspace Location enumerate
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceLocation {
    NotRecognize,
    Local,
    Remote,
}

/// Implement default associate function for Workspace Location enumerate
impl WorkspaceLocation {
    /// Default value of WorkspaceLocation
    #[allow(dead_code)]
    fn default() -> Self {
        WorkspaceLocation::NotRecognize
    }
}

/// An explicit conversion from a &str to WorkspaceLocation
impl From<&str> for WorkspaceLocation {
    /// Generate WorkspaceLocation from &str
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

/// Implement Hash for HashSet. Make Workspace a hashable type.
impl Hash for Workspace {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

/// Implement default associate function for Workspace Location enumerate
impl Workspace {
    #[allow(dead_code)]
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
        let decode_path = decode(raw_path.as_str()).expect("UTF-8").to_string();
        Workspace {
            path: raw_path.clone(),
            decode_path: decode_path.clone(),
            location_type: raw_path.as_str().into(),
            title: decode_path
                .clone()
                .split("/")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .to_string(),
        }
    }

    /// Strip uri prefix of decoded workspace path
    pub fn strip_decode_path(&self) -> String {
        let strip_uri_prefix = Regex::new(r"(file|vscode-remote):[/]+").unwrap();
        strip_uri_prefix.replace(&self.decode_path, "").to_string()
    }
}

/// Implement PartialEq for Workspace
///
/// Use Workspace's original path as the crucial condiction
impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl From<WorkspaceJson> for Workspace {
    fn from(_wj: WorkspaceJson) -> Self {
        let decode_folder_path = decode(_wj.folder.as_str()).expect("UTF-8").to_string();
        let location = WorkspaceLocation::from(_wj.folder.as_str());

        Workspace {
            path: _wj.folder,
            decode_path: decode_folder_path.clone(),
            location_type: location,
            title: decode_folder_path
                .clone()
                .split("/")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .to_string(),
        }
    }
}

impl From<&str> for Workspace {
    fn from(raw_path: &str) -> Self {
        let decode_folder_path = decode(raw_path).expect("UTF-8").to_string();

        let location = WorkspaceLocation::from(raw_path);

        Workspace {
            path: raw_path.to_string(),
            decode_path: decode_folder_path.clone(),
            location_type: location,
            title: decode_folder_path
                .clone()
                .split("/")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .to_string(),
        }
    }
}
