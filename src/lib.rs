#[macro_use]pub mod util;
pub mod cli;
pub mod search;
pub mod file;
mod line;
mod folder;
extern crate colored;

use crate::util::Result;
use crate::line::Line;
use std::io::BufRead;
use std::io::Write;
use colored::Colorize;
use atty::Stream;

pub fn process_folder<P>(root: P, options: &cli::Options, file_data: &mut file::Data) -> Result<()>
where P: AsRef<std::path::Path>
{
    let paths = folder::Scanner::new(root, &options)?.scan()?;

    if file_data.search_pattern_re_opt.is_some() {
        for path in &paths {
            process_file(path, options, file_data)?;
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

pub fn process_file(path: &std::path::PathBuf, options: &cli::Options, file_data: &mut file::Data) -> Result<()> {
    match file_data.load(path) {
        Err(_) => if options.verbose_level >= 1 {
            println!("Warning: Skipping \"{}\", could not load file", path.display());
        },

        Ok(()) => {
            file_data.split_in_lines()?;
            if file_data.search() {
                if options.output_filenames_only {
                    if options.null_separated_output {
                        print!("{}\0", format!("{}", file_data.path.display()));
                    } else {
                        println!("{}", format!("{}", file_data.path.display()));
                    }
                } else {
                    println!("{}", format!("{}", file_data.path.display()).green().bold());
                    let content = file_data.content.as_slice();
                    //Iterator that is meant to be options.output_before behind the
                    //one driving the for loop. `delay` indicates the actual delay.
                    let mut delayed_line_iter = file_data.lines.iter();
                    let mut delay = 0;
                    //As long as output_count is Some(>0), we will output
                    let mut output_count = None;
                    for line in file_data.lines.iter() {
                        if !line.matches.is_empty() {
                            output_count = Some(delay+options.output_after+1);
                        }

                        if let Some(cnt) = output_count {
                            let delayed_line = delayed_line_iter.next().unwrap();
                            if cnt > 0 {
                                delayed_line.print_colored(delayed_line.as_slice(content), &file_data.replace_opt);
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

                if file_data.replace_opt.is_some() {
                    if !options.simulate_replace {
                        file_data.replace_and_write()?;
                    }
                }
            }
        },
    }

    Ok(())
}

pub fn process_stdin(options: &cli::Options) -> Result<()> {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let (mut stdin_handle, mut stdout_handle) = (stdin.lock(), stdout.lock());
    let search_pattern_re_opt = match &options.search_pattern_opt {
        None => None,
        Some(str) => Some(search::create_regex(str, options.word_boundary, options.case_sensitive)?),
    };
    let stdout_is_tty = atty::is(Stream::Stdout);

    let mut buffer: Vec<u8> = vec![];
    let mut buffer_replaced: Vec<u8> = vec![];
    let mut line_nr = 0;

    while let Ok(size) = stdin_handle.read_until(0x0a_u8, &mut buffer) {
        if size == 0 {
            break;
        }

        line_nr += 1;

        let mut line = Line::new(line_nr, 0, buffer.len());

        let found_match = search_pattern_re_opt.as_ref().map_or(false, |re|line.search_for(re, &buffer));

        if stdout_is_tty {
            //When output is the console, we only output matches
            if found_match {
                line.print_colored(&buffer, &options.replace_opt);
            }
        } else {
            //When output is redirected, we output every line, regardless if it maches or not
            if found_match && options.replace_opt.is_some() {
                line.replace_with(&buffer, options.replace_opt.as_ref().unwrap(), &mut buffer_replaced);
                stdout_handle.write(&buffer_replaced)?;
            } else {
                stdout_handle.write(&buffer)?;
            }
        }

        buffer.clear();
    }
    Ok(())
}
