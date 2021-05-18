mod cli;

fn main() {
    match my_main() {
        Err(error) => {
            println!("Error: {}", error.descr);
            println!("Something went wrong");
        }
        Ok(()) => {
            println!("Everything went OK");
        }
    }
}

fn my_main() -> Result<(), cli::Error> {
    let mut options = cli::Options::new();
    println!("{:?}", options);

    let args:Vec<String> = std::env::args().skip(1).collect();
    options.parse(&args)?;
    println!("{:?}", options);

    if options.print_help {
        println!("{}", options.help());
    }

    Ok(())
}
