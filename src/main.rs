mod res;
mod cli;

fn my_main() -> Result<(), res::Error> {
    let mut options = cli::Options::new();
    println!("{:?}", options);

    options.parse(cli::args())?;
    println!("{:?}", options);

    if options.print_help {
        println!("{}", options.help());
    }

    Ok(())
}

fn main() {
    match my_main() {
        Err(error) => {
            res::print_error(error);
            println!("Something went wrong");
        }
        Ok(()) => {
            println!("Everything went OK");
        }
    }
}

