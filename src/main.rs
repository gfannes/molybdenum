#[macro_use ]mod res;
mod cli;
mod folder;

fn main() -> res::Result<()> {
    let mut options = cli::Options::new();

    options.parse(cli::args())?;
    if options.verbose_level > 0 {
        println!("{:?}", options);
    }

    if options.print_help {
        println!("{}", options.help());
    }

    let folder_scanner = folder::Scanner::new(&options);

    let paths = folder_scanner.scan()?;

    if options.output_filenames_only {
        for path in &paths {
            println!("{}", path.display());
        }
    }

    Ok(())
}