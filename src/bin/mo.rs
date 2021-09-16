extern crate molybdenum;
use molybdenum::cli;
use molybdenum::util;
use molybdenum::file;
use molybdenum::search;
use atty::Stream;

fn main() -> util::Result<()> {
    let mut options = cli::Options::new();

    options.parse(cli::args())?;
    if options.verbose_level >= 1 {
        println!("{:?}", options);
    }

    if let Some(b) = options.console_output_always {
        colored::control::set_override(b)
    }

    if options.output_help {
        println!("{}", options.help());
        return Ok(());
    }

    let stdin_is_console = atty::is(Stream::Stdin);
    if options.verbose_level >= 1 {
        println!("Stdin is a {}", if stdin_is_console {"console"} else {"redirection"});
    }

    let input_from_file = options.input_from_file_opt.unwrap_or(stdin_is_console);
    if input_from_file {
        if options.verbose_level >= 1 {
            println!("Taking input from file");
        }

        let mut search_pattern_re_opt = None;
        if let Some(search_pattern_str) = &options.search_pattern_opt {
            search_pattern_re_opt = Some(search::create_regex(search_pattern_str, options.word_boundary, options.case_sensitive)?);
        }
        let mut file_data = file::Data::new(search_pattern_re_opt, options.invert_pattern, &options.replace_opt);

        if options.roots.is_empty() {
            molybdenum::process_folder(".", &options, &mut file_data)?;
        } else {
            for root in &options.roots {
                let root = std::path::PathBuf::from(root);
                if root.is_dir() {
                    molybdenum::process_folder(&root, &options, &mut file_data)?;
                } else {
                    molybdenum::process_file(&root, &options, &mut file_data)?;
                }
            }
        }
    } else {
        if options.verbose_level >= 1 {
            println!("Taking input from Stdin");
        }

        molybdenum::process_stdin(&options)?;
    }

    Ok(())
}
