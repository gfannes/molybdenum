use crate::util::{Range};
use std::str::from_utf8;
use regex::bytes::Regex;
use colored::Colorize;

pub type Content = Vec<u8>;
type ContentSlice = [u8];

pub struct Line {
    pub nr: u64,
    pub range: Range,
    pub matches: Vec<Range>,
}
impl Line {
    //(start, size) indicate a part from some ContentSlice
    pub fn new(nr: u64, start: usize, size: usize) -> Line {
        Line{
            nr,
            range: start..start+size,
            matches: vec![],
        }
    }

    pub fn as_slice<'a>(&self, s: &'a ContentSlice) -> &'a ContentSlice {
        &s[self.range.clone()]
    }

    pub fn search_for(&mut self, re: &Regex, content: &ContentSlice) -> bool {
        self.matches.clear();
        let mut found_match = false;
        for m in re.find_iter(self.as_slice(content)) {
            self.matches.push(m.start()..m.end());
            found_match = true;
        }
        found_match
    }

    pub fn print_colored<'a>(&self, content: &ContentSlice, replace: &Option<&'a str>) {
        let my_print = |repl: &Option<&str>|{
            print!("{}:", format!("{}", self.nr).yellow());
            let mut offset = 0;
            for r in self.matches.iter() {
                if let Ok(normal_str) = from_utf8(&content[offset..r.start]) {
                    if let Some(highlight_str) = repl {
                        print!("{}{}", normal_str, highlight_str.on_purple());
                    } else if let Ok(highlight_str) = from_utf8(&content[r.start..r.end]) {
                        print!("{}{}", normal_str, highlight_str.bright_cyan().bold());
                    }
                }
                offset = r.end;
            }
            if let Ok(normal_str)= from_utf8(&content[offset..]) {
                print!("{}", normal_str);
            }
        };

        my_print(&None);
        if replace.is_some() {
            my_print(replace);
        }
    }
    
    pub fn replace_with(&self, content: &ContentSlice, replace: &str, output: &mut Vec<u8>) {
        output.clear();
        let mut offset = 0;
        for r in self.matches.iter() {
            output.extend_from_slice(&content[offset..r.start]);
            output.extend_from_slice(replace.as_bytes());
            offset = r.end;
        }
        output.extend_from_slice(&content[offset..]);
    }
}