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

    let folder_scanner = folder::Scanner::new(&options.root_folder);

    let paths = folder_scanner.scan()?;

    for path in &paths {
        println!("{}", path.display());
    }

    Ok(())
}
