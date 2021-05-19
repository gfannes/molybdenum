use crate::res::{Result,MyError};
use std::collections::VecDeque;

//Represents parsed CLI options
#[derive(Debug)]
pub struct Options {
    pub print_help: bool,
    pub input_filename: String,
}

//Creates a Vec of CLI option handlers
fn generate_option_vec() -> Vec<Option> {
    let mut v = Vec::new();

    v.push(Option::new("-h", "--help", "Print this help", Handler::Args0(|options|{
        options.print_help = true;
        Ok(())
    })));

    v.push(Option::new("-i", "--input-filename", "Input filename", Handler::Args1(|options, filename|{
        options.input_filename = filename.to_string();
        Ok(())
    })));

    v
}

//Represents raw CLI arguments as provided by the user
pub type Args = VecDeque<String>;

pub fn args() -> Args {
    std::env::args().skip(1).collect()
}

impl Options {
    pub fn new() -> Options {
        Options {
            print_help: false,
            input_filename: String::new(),
        }
    }

    pub fn parse(&mut self, mut args: Args) -> Result<()> {
        let options = generate_option_vec();

        while let Some(arg0) = args.pop_front() {
            for option in &options {
                //Check if arg0 matches with this Option
                if !option.suit(&arg0) {
                    continue;
                }

                //Call the handler, taking care of its amount of arguments
                match option.handler {
                    Handler::Args0(ftor) => {
                        ftor(self)?;
                    },
                    Handler::Args1(ftor) => {
                        //Only if there is an additional arg after arg0
                        if let Some(arg1)= args.get(0) {
                            ftor(self, arg1)?;
                        }
                    },
                }
            }
        }

        Ok(())
    }

    pub fn help(&self) -> String {
        let mut s = String::new();

        s.push_str("Help for the Molybdenum Searcher (mo):\n");
        for o in generate_option_vec() {
            s.push_str(&o.help());
            s.push_str("\n");
        }
        s.push_str("Created by Geert Fannes\n");

        s
    }
}

//Handler for the raw CLI arguments
enum Handler {
    Args0(Handler0),
    Args1(Handler1),
}
type Handler0 = fn(&mut Options) -> Result<()>;
type Handler1 = fn(&mut Options, &str) -> Result<()>;

//Represents a CLI option entity
struct Option {
    sh: &'static str,
    lh: &'static str,
    descr: &'static str,
    handler: Handler,
}

impl Option {
    fn new(sh:&'static str, lh:&'static str, descr:&'static str, handler: Handler) -> Option {
        let o = Option{sh, lh, descr, handler};
        o
    }

    fn help(&self) -> String {
        format!("\t{}\t{}\t{}", self.sh, self.lh, self.descr)
    }

    fn suit(&self, arg: &str) -> bool {
        self.sh == arg || self.lh == arg
    }
}
