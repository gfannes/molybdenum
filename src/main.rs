#[macro_use ]mod res;
mod cli;
mod folder;
mod pattern;
extern crate colored;

use crate::res::{MyError};
use regex::Regex;

use colored::*;

fn main() -> res::Result<()> {
    let mut options = cli::Options::new();

    options.parse(cli::args())?;
    if options.verbose_level >= 1 {
        println!("{:?}", options);
    }

    if options.output_help {
        println!("{}", options.help());
        return Ok(());
    }

    let folder_scanner = folder::Scanner::new(&options);

    let paths = folder_scanner.scan()?;

    if let Some(search_pattern_str) = &options.search_pattern_str {
        match Regex::new(search_pattern_str) {
            Err(_) => fail!("Search pattern \"{}\" is not a valid regex", search_pattern_str),

            Ok(seach_pattern_re) => {
                let mut pattern_searcher = pattern::Searcher::new(&seach_pattern_re);

                for path in &paths {
                    let f = std::fs::File::open(path)?;
                    if pattern_searcher.search(&mut std::io::BufReader::new(f)) {
                        let path_str = format!("{}", path.display());
                        println!("{} => {} matches", path_str.green(), pattern_searcher.matches.len());
                        for m in &pattern_searcher.matches {
                            println!("{}:{:?}", m.line_nr, m.content);
                        }
                        println!("");
                    }
                }
            }
        }
    } else {
        for path in &paths {
            println!("{}", path.display());
        }
    }

    Ok(())
}
