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

/// Reads a file at a specified path into a string.
pub fn read_to_string(path: &str) -> String {
    std::fs::read_to_string(path).expect(&*format! {"Failed to read {}", path})
}
