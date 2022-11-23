use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WorkspaceJson {
    pub folder: String,
}

#[cfg(test)]
mod test_workspacejson {
    use super::WorkspaceJson;

    #[test]
    fn test_workspacejson_init() {
        let _w = WorkspaceJson {
            folder: String::from("folder"),
        };
        assert_eq!(_w.folder, String::from("folder"));
    }

    #[test]
    fn test_workspacejson_is_deserializeable() {
        let _w = WorkspaceJson {
            folder: String::from("folder"),
        };

        let _ww: WorkspaceJson = serde_json::from_str(
            "{
                \"folder\": \"folder\"
            }",
        )
        .unwrap();

        assert_eq!(_ww.folder, _w.folder);
    }
}
