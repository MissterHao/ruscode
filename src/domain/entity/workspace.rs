use super::tag::Tag;

#[derive(Debug)]
pub struct Workspace {
    path: String,
    tags: Vec<Tag>,
}

impl Workspace {
    pub fn new() -> Self {
        Workspace {
            path: String::from("todo!()"),
            tags: vec![],
        }
    }
}
