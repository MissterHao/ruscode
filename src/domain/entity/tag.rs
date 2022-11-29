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

#[cfg(test)]
mod test_tag_entity {
    use super::Tag;

    #[test]
    fn tag_entity_should_create_successfully_with_new_associate_function() {
        assert_eq!(Tag::new().name, String::from(""));
    }

    #[test]
    fn tag_entity_should_create_successfully_with_init() {
        let t = Tag {
            name: String::new(),
        };
        assert_eq!(t.name, String::new());
    }

    #[test]
    fn tag_entity_should_print_successfully_in_debug_mode() {
        let t = Tag::new();
        format!("{:?}", t);
    }
}
