pub fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or_else(|| input.strip_suffix('\n'))
        .unwrap_or(input)
}

#[cfg(test)]
mod test_text {
    use super::strip_trailing_newline;

    #[test]
    fn strref_should_strip_newline() {
        let input = "This is a sentence with newline\r\n";
        let input2 = "This is a sentence with newline\n";

        assert_eq!(
            strip_trailing_newline(input),
            "This is a sentence with newline"
        );
        assert_eq!(
            strip_trailing_newline(input2),
            "This is a sentence with newline"
        );
    }
}
