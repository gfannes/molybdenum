use crate::res::{Result,MyError};
use std::io::Read;
use std::io::BufRead;
use std::path::PathBuf;
use regex::Regex;
use colored::Colorize;

type Range = std::ops::Range<usize>;

pub struct Line {
    pub nr: u64,
    range: Range,
    pub matches: Vec<Range>,
}
impl Line {
    pub fn as_str<'a>(&self, s: &'a str) -> &'a str {
        &s[self.range.clone()]
    }
    pub fn print_colored(&self, content: &str) {
        print!("{}:", format!("{}", self.nr).yellow());
        let mut offset = 0;
        for r in self.matches.iter() {
            print!("{}{}", &content[offset..r.start], &content[r.start..r.end].blue());
            offset = r.end;
        }
        print!("{}", &content[offset..]);
    }
}

pub struct Data {
    pub path: PathBuf,
    pub content: String,
    pub lines: Vec<Line>,

    tmp_str_: String,
}

impl Data {
    pub fn new() -> Data {
        Data{
            path: PathBuf::new(),
            content: String::new(),
            lines: vec![],

            tmp_str_: String::new(),
        }
    }

    pub fn load<P>(&mut self, path: P) -> Result<()>
        where P: AsRef<std::path::Path>
        {
            self.path = PathBuf::from(path.as_ref());

            let mut f = std::fs::File::open(&self.path)?;
            let md = f.metadata()?;
            let md_size = md.len() as usize;
            self.content.reserve(md_size);
            self.content.clear();
            let act_size = f.read_to_string(&mut self.content)?;
            assert_eq!(md_size, act_size);

            self.lines.clear();

            Ok(())
        }

    pub fn split_in_lines(&mut self) -> Result<()> {
        let mut content_str = self.content.as_str();
        let mut start_ix = 0;
        let mut line_nr = 0;
        while !content_str.is_empty() {
            line_nr += 1;

            let size = match content_str.find('\n') {
                None => content_str.len(),
                Some(ix) => ix+1,
            };

            self.lines.push(Line{
                nr: line_nr,
                range: start_ix..start_ix+size,
                matches: vec![],
            });
            content_str = &content_str[size..];
            start_ix += size;
        }
        Ok(())
    }

    pub fn search(&mut self, re: &regex::Regex) -> bool {
        let content = &self.content;
        let mut found_match = false;
        for line in self.lines.iter_mut() {
            for m in re.find_iter(line.as_str(content)) {
                line.matches.push(m.start()..m.end());
                found_match = true;
            }
        }
        found_match
    }
}

#[test]
pub fn test_file() -> Result<()> {
    let mut data = Data::new();

    data.load(file!())?;
    println!("Read {} bytes", data.content.len());

    data.split_in_lines()?;
    println!("Found {} lines", data.lines.len());

    let re = regex::Regex::new(r"regex")?;
    assert!(data.search(&re));

    for line in &data.lines {
        if !line.matches.is_empty() {
            println!("{} {}", line.nr, line.matches.len());
        }
    }

    Ok(())
}
