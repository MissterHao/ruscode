#[derive(Debug)]
#[allow(dead_code)]
pub struct Tag {
    name: String,
}

impl Tag {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Tag {
            name: String::from(""),
        }
    }
}
