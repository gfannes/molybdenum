use crate::util::{Result, MyError};
use regex::bytes::{Regex, RegexBuilder};

pub fn create_regex(pattern: &str, word_boundary: bool, case_sensitive: bool) -> Result<Regex> {
    let mut pattern = pattern.to_string();
    if word_boundary {
        pattern = format!("\\b{}\\b", pattern);
    }

    let re = match RegexBuilder::new(&pattern).case_insensitive(!case_sensitive).build() {
        Err(_) => fail!("Pattern \"{}\" is not a valid regex", pattern),
        Ok(re) => re,
    };

    Ok(re)
}
