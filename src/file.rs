use crate::res::Result;
use std::io::Read;
use std::path::PathBuf;
use std::str::from_utf8;
use regex::bytes::Regex;
use colored::Colorize;

// type Content = String;
// type ContentSlice = str;
type Content = Vec<u8>;
type ContentSlice = [u8];

type Range = std::ops::Range<usize>;

pub struct Line {
    pub nr: u64,
    range: Range,
    pub matches: Vec<Range>,
}
impl Line {
    pub fn as_slice<'a>(&self, s: &'a ContentSlice) -> &'a ContentSlice {
        &s[self.range.clone()]
    }
    pub fn print_colored(&self, content: &ContentSlice) {
        print!("{}:", format!("{}", self.nr).yellow());
        let mut offset = 0;
        for r in self.matches.iter() {
            if let Ok(a_str) = from_utf8(&content[offset..r.start]) {
                if let Ok(b_str) = from_utf8(&content[r.start..r.end]) {
                    print!("{}{}", a_str, b_str.blue());
                }
            }
            offset = r.end;
        }
        if let Ok(a_str)= from_utf8(&content[offset..]) {
            print!("{}", a_str);
        }
    }
}

pub struct Data {
    pub path: PathBuf,
    pub content: Content,
    pub lines: Vec<Line>,
}

impl Data {
    pub fn new() -> Data {
        Data{
            path: PathBuf::new(),
            content: Content::new(),
            lines: vec![],
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
            // let act_size = f.read_to_string(&mut self.content)?;
            let act_size = f.read_to_end(&mut self.content)?;
            assert_eq!(md_size, act_size);

            self.lines.clear();

            Ok(())
        }

    pub fn split_in_lines(&mut self) -> Result<()> {
        let mut content = self.content.as_slice();
        let mut start_ix = 0;
        let mut line_nr = 0;
        while !content.is_empty() {
            line_nr += 1;

            let size = match content.iter().position(|&v|{v==0x0a_u8}) {
                None => content.len(),
                Some(ix) => ix+1,
            };

            self.lines.push(Line{
                nr: line_nr,
                range: start_ix..start_ix+size,
                matches: vec![],
            });
            content = &content[size..];
            start_ix += size;
        }
        Ok(())
    }

    pub fn search(&mut self, re: &Regex) -> bool {
        let content = &self.content;
        let mut found_match = false;
        for line in self.lines.iter_mut() {
            for m in re.find_iter(line.as_slice(content)) {
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

    let re = Regex::new(br"regex")?;
    assert!(data.search(&re));

    for line in &data.lines {
        if !line.matches.is_empty() {
            println!("{} {}", line.nr, line.matches.len());
        }
    }

    Ok(())
}
