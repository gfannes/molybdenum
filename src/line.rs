use crate::util::{Range};
use crate::search::{Search, Replace};
use std::str::from_utf8;
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

    pub fn search_for(&mut self, search: &Search, content: &ContentSlice) -> bool {
        self.matches.clear();
        let mut found_match = false;
        for m in search.regex.find_iter(self.as_slice(content)) {
            self.matches.push(m.start()..m.end());
            found_match = true;
        }
        found_match
    }

    pub fn print_colored(&self, content: &ContentSlice, search: &Search, replace_opt: &Option<Replace>) {
        let my_print = |replace_opt: &Option<Replace>|{
            print!("{}:", format!("{}", self.nr).yellow());
            let mut offset = 0;
            for r in self.matches.iter() {
                if let Ok(normal_str) = from_utf8(&content[offset..r.start]) {
                    if let Ok(match_str) = from_utf8(&content[r.start..r.end]) {
                        print!("{}", normal_str);
                        match &replace_opt {
                            None => print!("{}", match_str.bright_cyan().bold()),
                            Some(replace) => {
                                let caps = search.regex.captures(match_str.as_bytes()).unwrap();
                                for (capture_ix, part) in &replace.parts {
                                    if *capture_ix >= 0 {
                                        print!("{}", from_utf8(caps.get(*capture_ix as usize).unwrap().as_bytes()).unwrap().on_purple());
                                    }
                                    print!("{}", part.on_purple());
                                }
                            },
                        }
                    }
                }
                offset = r.end;
            }
            if let Ok(normal_str) = from_utf8(&content[offset..]) {
                if replace_opt.is_none() || replace_opt.as_ref().unwrap().prefix.is_none() {
                    print!("{}", normal_str);
                }
            }
        };

        my_print(&None);
        if replace_opt.is_some() {
            my_print(replace_opt);
        }
    }

    pub fn print_colored_match(&self, content: &ContentSlice) {
        //@todo: make configurable
        // print!("{}:", format!("{}", self.nr).yellow());
        for r in self.matches.iter() {
            if let Ok(match_str) = from_utf8(&content[r.start..r.end]) {
                print!("{}", match_str.bright_cyan().bold());
            }
        }
        println!("");
    }
    
    pub fn replace_with(&self, content: &ContentSlice, search: &Search, replace: &Replace, output: &mut Vec<u8>) {
        output.clear();
        let mut offset = 0;
        for r in self.matches.iter() {
            output.extend_from_slice(&content[offset..r.start]);
            let match_bytes = &content[r.start..r.end];
            let caps = search.regex.captures(match_bytes).unwrap();
            for (capture_ix, part) in &replace.parts {
                if *capture_ix >= 0 {
                    output.extend_from_slice(caps.get(*capture_ix as usize).unwrap().as_bytes());
                }
                output.extend_from_slice(part.as_bytes());
            }
            offset = r.end;
        }
        output.extend_from_slice(&content[offset..]);
    }
    
    pub fn only_matches(&self, content: &ContentSlice, output: &mut Vec<u8>) {
        output.clear();
        for r in self.matches.iter() {
            output.extend_from_slice(&content[r.start..r.end]);
        }
    }
}
