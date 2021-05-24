#[macro_use ]mod res;
mod cli;
mod folder;
mod file;
extern crate colored;

use crate::res::{MyError};
use regex::bytes::Regex;

use colored::Colorize;

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

    let folder_scanner = folder::Scanner::new(&options)?;

    let paths = folder_scanner.scan()?;

    if let Some(search_pattern_str) = &options.search_pattern_str {

        //Adjust search pattern to word boundary, if needed
        let search_pattern_str = if options.word_boundary { format!("\\b{}\\b", search_pattern_str) } else { search_pattern_str.to_string() };

        match Regex::new(&search_pattern_str) {
            Err(_) => fail!("Search pattern \"{}\" is not a valid regex", search_pattern_str),

            Ok(seach_pattern_re) => {
                let mut file_data = file::Data::new();

                let replace: Option<&str> = options.replace_str.as_ref().map(|r|r.as_str());

                for path in &paths {
                    match file_data.load(path) {
                        Err(_) => if options.verbose_level >= 1 {
                            println!("Warning: Skipping file \"{}\", it contains non-UTF8 characters", path.display());
                        },
                        Ok(()) => {
                            file_data.split_in_lines()?;
                            if file_data.search(&seach_pattern_re) {
                                println!("{} {}", format!("{}", file_data.path.display()).green().bold(), file_data.lines.len());

                                if !options.output_filenames_only {
                                    let content = file_data.content.as_slice();
                                    for line in file_data.lines.iter() {
                                        if !line.matches.is_empty() {
                                            line.print_colored(line.as_slice(content), &replace);
                                        }
                                    }
                                    println!("");
                                }

                                if let Some(repl) = replace {
                                    if !options.simulate_replace {
                                        file_data.write(repl)?;
                                    }
                                }
                            }
                        },
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
