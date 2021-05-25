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
                                if options.output_filenames_only {
                                    if options.null_separated_output {
                                        print!("{}\0", format!("{}", file_data.path.display()));
                                    } else {
                                        println!("{}", format!("{}", file_data.path.display()));
                                    }
                                } else {
                                    println!("{} {}", format!("{}", file_data.path.display()).green().bold(), file_data.lines.len());
                                    let content = file_data.content.as_slice();
                                    //Iterator that is meant to be options.output_before behind the
                                    //one driving the for loop. `delay` indicates the actual delay.
                                    let mut delayed_line_iter = file_data.lines.iter();
                                    let mut delay = 0;
                                    //As long as output_count is Some(>0), we will output
                                    let mut output_count = None;
                                    for (ix, line) in file_data.lines.iter().enumerate() {
                                        if !line.matches.is_empty() {
                                            output_count = Some(delay+options.output_after+1);
                                        }

                                        if let Some(cnt) = output_count {
                                            let delayed_line = delayed_line_iter.next().unwrap();
                                            if cnt > 0 {
                                                delayed_line.print_colored(delayed_line.as_slice(content), &replace);
                                                output_count = Some(cnt-1);
                                            } else {
                                                println!("...");
                                                output_count = None;
                                            }
                                        } else if delay < options.output_before {
                                            delay += 1;
                                        } else {
                                            let _ = delayed_line_iter.next();
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
            if options.null_separated_output {
                print!("{}\0", format!("{}", path.display()));
            } else {
                println!("{}", format!("{}", path.display()));
            }
        }
    }

    Ok(())
}
