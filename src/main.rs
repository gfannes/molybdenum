mod res;
mod cli;

fn main() -> res::Result<()> {
    let mut options = cli::Options::new();

    options.parse(cli::args())?;
    if options.verbose_level > 0 {
        println!("{:?}", options);
    }

    if options.print_help {
        println!("{}", options.help());
    }

    let mut path = std::path::PathBuf::new();
    path.push(".");
    visit_dir(path);

    Ok(())
}

fn my_is_hidden(p: &std::ffi::OsStr) -> bool {
    if let Some(s) = p.to_str() {
        if let Some(ch) = s.chars().next() {
            ch == '.'
        } else {
            false
        }
    } else {
        false
    }
}

fn visit_dir(parent_path: std::path::PathBuf) -> res::Result<()> {

    for entry in std::fs::read_dir(parent_path)? {
        let path = entry?.path();
        let filename = path.file_name().unwrap();
        println!("{:?} {:?}", path, filename);
        if my_is_hidden(&filename) {
            println!(" => This is hidden");
            continue;
        }

        visit_dir(path);
    }

    Ok(())
}
