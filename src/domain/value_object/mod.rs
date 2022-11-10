use serde::Deserialize;

#[derive(Deserialize)]
pub struct WorkspaceJson {
    pub folder: String,
    // #[serde(default)]
    // is_remote: Option<FileLocation>,

    // #[serde(default = "string_default")]
    // decode_folder: String,
}

// impl WorkspaceJson {
//     pub fn clean(&mut self) {
//         self.decode_folder = decode(self.folder.as_str()).expect("UTF-8").to_string();
//         self.is_remote = if self.folder.starts_with("file:///") {
//             Some(FileLocation::Local)
//         } else if self.folder.starts_with("vscode-remote://") {
//             Some(FileLocation::Remote)
//         } else {
//             // Default
//             Some(FileLocation::Local)
//         };
//     }
// }
