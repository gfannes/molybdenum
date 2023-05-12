use crate::line::{Content, Line};
use crate::search::{Replace, Search};
use crate::util::{MyError, Result};
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

pub struct Data {
    pub search_opt: Option<Search>,
    pub invert_pattern: bool,
    pub replace_opt: Option<Replace>,
    pub path: PathBuf,
    pub content: Content,
    pub lines: Vec<Line>,
}

impl Data {
    pub fn new(
        search_opt: Option<Search>,
        invert_pattern: bool,
        replace_opt: Option<Replace>,
    ) -> Data {
        Data {
            search_opt,
            invert_pattern,
            replace_opt: replace_opt,
            path: PathBuf::new(),
            content: Content::new(),
            lines: vec![],
        }
    }

    pub fn load<P>(&mut self, path: P) -> Result<()>
    where
        P: AsRef<std::path::Path>,
    {
        self.path = PathBuf::from(path.as_ref());

        let mut f = std::fs::File::open(&self.path)?;
        let md = f.metadata()?;
        let md_size = md.len() as usize;
        self.content.reserve(md_size);
        self.content.clear();
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

            let size = match content.iter().position(|&v| v == 0x0a_u8) {
                None => content.len(),
                Some(ix) => ix + 1,
            };

            self.lines.push(Line::new(line_nr, start_ix, size));
            content = &content[size..];
            start_ix += size;
        }
        Ok(())
    }

    pub fn search_for_matches(&mut self) -> bool {
        match &self.search_opt {
            None => false,

            Some(search) => {
                let content = &self.content;
                let mut found_match = false;
                for line in self.lines.iter_mut() {
                    found_match = line.search_for(&search, content) || found_match;
                }
                found_match
            }
        }
    }

    pub fn replace_and_write(&self) -> Result<()> {
        match &self.replace_opt {
            None => panic!("Expected a replace string here"),

            Some(replace) => {
                let mut f = std::fs::File::create(&self.path)?;
                let content_slice = &self.content;
                let search = self.search_opt.as_ref().unwrap();
                for line in self.lines.iter() {
                    let line_slice = line.as_slice(content_slice);
                    let mut offset = 0;
                    for r in line.matches.iter() {
                        f.write(&line_slice[offset..r.start])?;
                        {
                            let match_bytes = &line_slice[r.start..r.end];
                            let caps = search.regex.captures(match_bytes);
                            for (capture_ix, part) in &replace.parts {
                                if *capture_ix >= 0 {
                                    if caps.is_none() {
                                        fail!("Could not search for capture groups, but they are used here. This happens when a search with word boundary does not match in the substring match_str");
                                    }
                                    f.write(
                                        caps.as_ref()
                                            .unwrap()
                                            .get(*capture_ix as usize)
                                            .unwrap()
                                            .as_bytes(),
                                    )?;
                                }
                                f.write(part.as_bytes())?;
                            }
                        }
                        offset = r.end;
                    }
                    f.write(&line_slice[offset..])?;
                }
            }
        }
        Ok(())
    }
}

#[test]
pub fn test_file() -> Result<()> {
    use crate::search;

    let mut data = Data::new(search::Search::new("regex", false, false).ok(), false, None);

    data.load(file!())?;
    println!("Read {} bytes", data.content.len());

    data.split_in_lines()?;
    println!("Found {} lines", data.lines.len());

    assert!(data.search_for_matches());

    for line in &data.lines {
        if !line.matches.is_empty() {
            println!("{} {}", line.nr, line.matches.len());
        }
    }

    Ok(())
}
