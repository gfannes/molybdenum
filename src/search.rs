use crate::util::{Result, MyError};
use regex::bytes::Regex;

pub fn create_regex(pattern: &str, word_boundary: bool) -> Result<Regex> {
    let mut pattern = pattern.to_string();
    if word_boundary {
        pattern = format!("\\b{}\\b", pattern);
    }

    let re = match Regex::new(&pattern) {
        Err(_) => fail!("Pattern \"{}\" is not a valid regex", pattern),
        Ok(re) => re,
    };

    Ok(re)
}
