use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub enum SearchingStrategyType {
    All,
    Tags,
    PlainText,
    PlainTextMixTags,
}

impl SearchingStrategyType {
    pub fn parse(path_length: usize, tags_count: usize) -> SearchingStrategyType {
        match (path_length > 0, tags_count > 0) {
            (true, true) => SearchingStrategyType::PlainTextMixTags,
            (true, false) => SearchingStrategyType::PlainText,
            (false, true) => SearchingStrategyType::Tags,
            (false, false) => SearchingStrategyType::All,
        }
    }
}

/// Contain and Parse searching strategy information
#[derive(Debug, PartialEq, Eq)]
pub struct SearchingStrategy {
    path: String,
    tags: Vec<String>,
    pub searching_type: SearchingStrategyType,
}

impl SearchingStrategy {
    #[allow(dead_code)]
    pub fn default() -> SearchingStrategy {
        SearchingStrategy {
            path: String::new(),
            tags: vec![],
            searching_type: SearchingStrategyType::All,
        }
    }

    #[allow(dead_code)]
    pub fn new(origin: &str) -> SearchingStrategy {
        origin.into()
    }
}

impl From<String> for SearchingStrategy {
    fn from(origin: String) -> Self {
        origin.as_str().into()
    }
}

impl From<&str> for SearchingStrategy {
    fn from(origin: &str) -> Self {
        let tag_re = Regex::new(r"#[a-zA-Z0-9]+").unwrap();
        let tags = tag_re
            .captures_iter(origin)
            .map(|x| x.get(0).unwrap().as_str())
            .map(|x| x.to_string().replace(' ', "").replace('#', ""))
            .filter(|x| !x.is_empty())
            .collect::<Vec<String>>();

        let filtered_text = tag_re.replace_all(origin, "");
        let tags_count = tags.len();

        SearchingStrategy {
            path: String::from(filtered_text.clone()),
            searching_type: SearchingStrategyType::parse(filtered_text.clone().len(), tags_count),
            tags,
        }
    }
}

impl SearchingStrategy {
    // pub fn filter
}

#[cfg(test)]
mod test_searching_strategy_mod {

    use super::*;

    #[test]
    fn test_default_str_into_searching_strategy() {
        assert_eq!(SearchingStrategy::default(), "".into());
    }

    #[test]
    fn test_spaces_str_into_searching_strategy() {
        assert_eq!(
            SearchingStrategy {
                path: String::from("    "),
                tags: vec![],
                searching_type: SearchingStrategyType::PlainText
            },
            "    ".into()
        );
    }
    #[test]
    fn test_empty_tagname_str_into_searching_strategy() {
        assert_eq!(
            SearchingStrategy {
                path: String::from("    #"),
                tags: vec![],
                searching_type: SearchingStrategyType::PlainText
            },
            "    #".into()
        );
    }
    #[test]
    fn test_mix_tagname_str_into_searching_strategy() {
        assert_eq!(
            SearchingStrategy {
                path: String::from("    "),
                tags: vec![String::from("XD"), String::from("QQ")],
                searching_type: SearchingStrategyType::PlainTextMixTags
            },
            "#XD    #QQ".into()
        );
    }
    #[test]
    fn test_only_tag_str_into_searching_strategy() {
        assert_eq!(
            SearchingStrategy {
                path: String::from(""),
                tags: vec![String::from("XD"), String::from("ABC"), String::from("QQ")],
                searching_type: SearchingStrategyType::Tags
            },
            "#XD#ABC#QQ".into()
        );
    }

    /// String into searchingStrategy
    #[test]
    fn test_string_into_default_searching_strategy() {
        assert_eq!(SearchingStrategy::default(), String::from("").into());
    }
}
