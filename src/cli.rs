pub struct Error {
    pub descr: String,
}
impl Error {
    fn new(descr: &str) -> Error {
        Error{descr: descr.to_string()}
    }
}

#[derive(Debug)]
pub struct Options {
    pub print_help: bool,
    pub input_filename: String,
}

type Args = [String];
impl Options {
    pub fn new() -> Options {
        Options {
            print_help: false,
            input_filename: String::new(),
        }
    }

    pub fn parse(&mut self, mut args: &Args) -> Result<(), Error> {
        let options = Options::generate_options();

        while !args.is_empty() {
            let arg = &args[0];
            args = &args[1..];
            println!("Processing arg {}", arg);
            for o in &options {
                if o.sh == arg || o.lh == arg {
                    println!("  Found a match");
                    (o.oper)(self, &mut args)?;
                }
            }
        }
        Ok(())
    }

    pub fn help(&self) -> String {
        let mut s = String::new();
        s.push_str("Help for \"mo\":\n");
        for o in Options::generate_options() {
            s.push_str(&o.help());
            s.push_str("\n");
        }
        s.push_str("Created by Geert Fannes\n");
        s
    }

    fn generate_options() -> Vec<Option> {
        let mut v = Vec::<Option>::new();
        v.push(Option::new("-h", "--help", "Print this help", |options, _args|{options.print_help = true; Ok(())}));
        v.push(Option::new("-i", "--input", "Input filename", |options, args| -> Result<(), Error>{
            if args.is_empty() {
                Err(Error::new("Expected input filename"))
            } else {
                options.input_filename = args[0].to_string();
                *args = &args[1..];
                Ok(())
            }
        }));
        v
    }
}

type Oper = fn(&mut Options, &mut &Args) -> Result<(), Error>;
struct Option {
    sh: &'static str,
    lh: &'static str,
    descr: &'static str,
    oper: Oper,
}

impl Option {
    fn new(sh:&'static str, lh:&'static str, descr:&'static str, oper: Oper) -> Option {
        let o = Option{sh, lh, descr, oper};
        o
    }
    fn help(&self) -> String {
        format!("\t{}\t{}\t{}", self.sh, self.lh, self.descr)
    }
}
