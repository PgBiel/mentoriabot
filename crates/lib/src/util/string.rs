/// Limits the size of the given string to the given max length.
/// If the string is larger than the max length, its last 3 characters
/// before the max length is surpassed are replaced with three dots (...),
/// and the string is subsequently truncated to fit into the given max length.
pub fn limit_string_len(string: &str, max_len: usize) -> String {
    if string.len() > max_len {
        let mut result = format!("{}...", &string[..max_len.saturating_sub(3)]);
        result.truncate(max_len);
        result
    } else {
        string.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit_string_len_doesnt_affect_string_smaller_than_max_len() {
        assert_eq!(limit_string_len("0123456789", 15), "0123456789".to_owned());
    }

    #[test]
    fn test_limit_string_len_doesnt_affect_string_with_same_len_as_max_len() {
        assert_eq!(limit_string_len("0123456789", 10), "0123456789".to_owned());
    }

    #[test]
    fn test_limit_string_len_adds_three_dots_and_restricts_len_of_string_with_larger_len_than_max_len(
    ) {
        assert_eq!(limit_string_len("0123456789", 9), "012345...".to_owned());
    }

    #[test]
    fn test_limit_string_len_returns_three_dots_when_max_len_is_3() {
        assert_eq!(limit_string_len("0123456789", 3), "...".to_owned());
    }

    #[test]
    fn test_limit_string_len_returns_less_dots_when_max_len_is_less_than_3() {
        assert_eq!(limit_string_len("0123456789", 2), "..".to_owned());
        assert_eq!(limit_string_len("0123456789", 1), ".".to_owned());
        assert_eq!(limit_string_len("0123456789", 0), "".to_owned());
    }
}
