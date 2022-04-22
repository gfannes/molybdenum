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

pub struct Search {
    pub regex: Regex,
}
impl Search {
    pub fn new(pattern: &str, word_boundary: bool, case_sensitive: bool) -> Result<Search> {
        let mut pattern = pattern.to_string();
        if word_boundary {
            pattern = format!("\\b{}\\b", pattern);
        }

        let regex = match RegexBuilder::new(&pattern).case_insensitive(!case_sensitive).build() {
            Err(_) => fail!("Pattern \"{}\" is not a valid regex", pattern),
            Ok(regex) => regex,
        };

        let search = Search {
            regex,
        };

        Ok(search)
    }
}

pub struct Replace {
    pub repl: String,
    pub prefix: std::option::Option<String>,
    pub parts: Vec<(i32, String)>,
}
impl Replace {
    pub fn new(repl: &str, prefix_opt: &std::option::Option<String>) -> Replace {
        let mut res = Replace {
            repl: repl.to_string(),
            prefix: None,
            parts: vec![],
        };

        match prefix_opt {
            None => {
                res.parts = vec![(-1, repl.to_string())];
            },
            Some(prefix) => {
                let mut repl = repl;
                while !repl.is_empty() {
                    let capture_ix = match repl.strip_prefix(prefix) {
                        None => -1,
                        Some(rest) => {
                            repl = rest;
                            if repl.is_empty() {
                                1
                            } else {
                                let ix = match repl.chars().next().unwrap() {
                                    '0' => Some(0),
                                    '1' => Some(1),
                                    '2' => Some(2),
                                    '3' => Some(3),
                                    '4' => Some(4),
                                    '5' => Some(5),
                                    '6' => Some(6),
                                    '7' => Some(7),
                                    '8' => Some(8),
                                    '9' => Some(9),
                                    _ => None,
                                };
                                match ix {
                                    None => 1,
                                    Some(ix) => {
                                        repl = &repl[1..];
                                        ix
                                    }
                                }
                            }
                        },
                    };

                    let part;
                    match repl.find(prefix) {
                        None => {
                            part = repl.to_string();
                            repl = &repl[0..0];
                        },
                        Some(ix) => {
                            part = repl[..ix].to_string();
                            repl = &repl[ix..];
                        },
                    };

                    println!("{}: {}", capture_ix, part);
                    res.parts.push((capture_ix, part));
                }
            },
        }
        res
    }
}