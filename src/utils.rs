use fancy_regex::Regex;
use lazy_static::lazy_static;

#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Replaces internal whitespace with a single space, and removes leading and trailing whitespace.
pub fn truncate_whitespace(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s+",).unwrap();
    }
    RE.replace_all(s, " ").trim().to_string()
}

/// Redacts card numbers by replacing any sequence of 13 to 16 digits, delimited or otherwise,
/// with "**** **** **** ****".
/// Modified from https://stackoverflow.com/questions/2995476/replacing-credit-card-numbers,
/// to catch more delimiters in digit sequences.
/// We would rather redact sequences that are not actually card numbers (false positives)
/// than allow actual card numbers to go un-redacted (false negatives).
pub fn redact_card_numbers(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:\d[\W_]*?){13,16}").unwrap();
    }
    RE.replace_all(s, "**** **** **** ****").to_string()
}

/// Reads a file at a specified path into a string.
pub fn read_to_string(path: &str) -> String {
    std::fs::read_to_string(path).expect(&*format! {"Failed to read {}", path})
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1234567812345", "**** **** **** ****"; "when shorter")]
    #[test_case("1234567812345678", "**** **** **** ****"; "when concatenated")]
    #[test_case("1234-5678-1234-5678", "**** **** **** ****"; "when hyphenated")]
    #[test_case("1234_5678_1234_5678", "**** **** **** ****"; "when underscored")]
    #[test_case("1234 5678 1234 5678", "**** **** **** ****"; "when spaced apart")]
    #[test_case("1234 5678-1234_5678", "**** **** **** ****"; "when delimiters mixed")]
    #[test_case("\n1234567812345678\n", "\n**** **** **** ****\n"; "when on separate lines and concatenated")]
    #[test_case("\n1234-5678-1234-5678\n", "\n**** **** **** ****\n"; "when on separate lines and hyphenated")]
    #[test_case("\n1234_5678_1234_5678\n", "\n**** **** **** ****\n"; "when on separate lines and underscored")]
    #[test_case("\n1234 5678 1234 5678\n", "\n**** **** **** ****\n"; "when on separate lines and spaced apart")]
    #[test_case("\n1234 5678-1234_5678\n", "\n**** **** **** ****\n"; "when on separate lines and delimiters mixed")]
    #[test_case("abc1234567812345678abc", "abc**** **** **** ****abc"; "when surrounded by other characters and concatenated")]
    #[test_case("abc1234-5678-1234-5678abc", "abc**** **** **** ****abc"; "when surrounded by other characters and hyphenated")]
    #[test_case("abc1234_5678_1234_5678abc", "abc**** **** **** ****abc"; "when surrounded by other characters and underscored")]
    #[test_case("abc1234 5678 1234 5678abc", "abc**** **** **** ****abc"; "when surrounded by other characters and spaced apart")]
    #[test_case("abc1234 5678-1234_5678abc", "abc**** **** **** ****abc"; "when surrounded by other characters and delimiters mixed")]
    #[test_case("123456781234567812345678123456781234567812345678", "**** **** **** ******** **** **** ******** **** **** ****"; "when concatenated multiple times")]
    fn test_card_number_redaction(path: &str, expected: &str) {
        assert_eq!(redact_card_numbers(path), expected);
    }
}
