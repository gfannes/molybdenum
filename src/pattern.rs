use std::io::BufRead;
use regex::Regex;

//TODO: Add support for files with non-UTF8 content:
//* Use regex::bytes::Regex iso regex::Regex
//* Use Vec<u8> iso String
//* Use BufRead.read_until() iso BufRead.read_line()

pub struct Match {
    pub line_nr: u32,
    pub content: String,
}
pub type Matches = Vec<Match>;

pub struct Searcher<'a> {
    pub matches: Matches,

    search_re: &'a Regex,

    //Used to avoid re-allocating memory
    line_: String,
}

impl Searcher<'_> {
    pub fn new<'a>(search_re: &'a Regex) -> Searcher<'a> {
        Searcher{
            search_re,
            matches: Matches::new(),
            line_: String::new(),
        }
    }

    pub fn search(&mut self, input: &mut dyn BufRead) -> bool {
        let mut line_nr = 1u32;
        self.matches.clear();
        while self.read_line_(input) {
            if self.search_re.is_match(&self.line_) {
                self.matches.push(Match{line_nr, content: self.line_.clone()})
            }

            self.line_.clear();
            line_nr += 1;
        }
        !self.matches.is_empty()
    }

    fn read_line_(&mut self, input: &mut dyn BufRead) -> bool {
        self.line_.clear();
        match input.read_line(&mut self.line_) {
            Err(_) => false,
            Ok(size) => size > 0
        }
    }
}

#[test]
pub fn test_search() {
    let s = r"line 1
end 2".to_vec();

    let re = Regex::new(r"line").unwrap();

    let mut searcher = Searcher::new(&re);

    searcher.search(&mut&s[..]);
}
