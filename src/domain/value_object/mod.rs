use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WorkspaceJson {
    pub folder: String,
}
