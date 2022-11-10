#[derive(Debug)]
pub(crate) struct Tag {
    name: String
}

impl Tag {
    pub fn new() -> Self {
        Tag {
            name: String::from("")
        }
    }
}