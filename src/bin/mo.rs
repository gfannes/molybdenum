extern crate molybdenum;
use molybdenum::cli;
use molybdenum::util;
use atty::Stream;

fn main() -> util::Result<()> {
    let mut options = cli::Options::new();

    options.parse(cli::args())?;
    if options.verbose_level >= 1 {
        println!("{:?}", options);
    }

    if options.output_help {
        println!("{}", options.help());
        return Ok(());
    }

    if atty::is(Stream::Stdin) {
        molybdenum::process_folders(&options)?;
    } else {
        molybdenum::process_stdin(&options)?;
    }

    Ok(())
}
