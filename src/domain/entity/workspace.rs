use super::tag::Tag;
use crate::domain::value_object::WorkspaceJson;
use urlencoding::decode;

#[derive(Debug)]
pub struct Workspace {
    path: String,
    pub decode_path: String,
    tags: Vec<Tag>,
    pub location_type: WorkspaceLocation,
}

impl Workspace {
    pub fn new() -> Self {
        Workspace {
            path: String::new(),
            decode_path: String::new(),
            tags: vec![],
            location_type: WorkspaceLocation::default(),
        }
    }
}

#[derive(Debug)]
pub enum WorkspaceLocation {
    NotRecognize,
    Local,
    Remote,
}

impl WorkspaceLocation {
    fn default() -> Self {
        WorkspaceLocation::NotRecognize
    }
}

impl From<WorkspaceJson> for Workspace {
    fn from(_wj: WorkspaceJson) -> Self {
        let decode_folder_path = decode(_wj.folder.as_str()).expect("UTF-8").to_string();

        let location = if _wj.folder.starts_with("file://") {
            WorkspaceLocation::Local
        } else if _wj.folder.starts_with("vscode-remote://") {
            WorkspaceLocation::Remote
        } else {
            // Default
            WorkspaceLocation::NotRecognize
        };

        Workspace {
            path: _wj.folder,
            decode_path: decode_folder_path,
            tags: vec![],
            location_type: location,
        }
    }
}
