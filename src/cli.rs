use crate::res::{Result,MyError};
use std::collections::VecDeque;

//Represents parsed CLI options
#[derive(Debug,PartialEq,Eq)]
pub struct Options {
    pub print_help: bool,
    pub input_filename: String,
    pub verbose_level: i32,
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

    v.push(Option::new("-V", "--verbose-level", "Verbosity level", Handler::Args1(|options, level|{
        options.verbose_level = my_to_i32(level, format!("Could not convert \"{}\" into a verbosity level", level))?;
        Ok(())
    })));

    v
}

fn my_to_i32(s: &str, fail_msg: String) -> Result<i32> {
    match s.parse::<i32>() {
        Ok(v) => Ok(v),
        _ => Err(MyError::create(&fail_msg)),
    }
}

//Represents raw CLI arguments as provided by the user
pub type Args = VecDeque<String>;

pub fn args() -> Args {
    std::env::args().skip(1).collect()
}

impl Options {
    pub fn default() -> Options {
        Options {
            print_help: false,
            input_filename: String::new(),
            verbose_level: 0,
        }
    }

    pub fn new() -> Options {
        Options::default()
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
                        if let Some(arg1) = args.get(0) {
                            ftor(self, arg1)?;
                        } else {
                            return Err(MyError::create(&format!("Option {} expects additional argument", option.lh)));
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
        s.push_str("Created by Geert Fannes");

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

#[test]
fn test_options_parse() {
    struct Scn {
        args: Vec<&'static str>,

        parse_ok: bool,
        options: Options,
    }

    let scns = [
        //Positive scenarios
        //Single option
        Scn{
            args: vec!["-h"],
            parse_ok: true,
            options: Options{print_help: true, ..Options::default()},
        },
        Scn{
            args: vec!["-i", "input_filename"],
            parse_ok: true,
            options: Options{input_filename: String::from("input_filename"), ..Options::default()},
        },
        Scn{
            args: vec!["-V", "3"],
            parse_ok: true,
            options: Options{verbose_level: 3, ..Options::default()},
        },
        //All options
        Scn{
            args: vec!["-h", "-i", "input_filename"],
            parse_ok: true,
            options: Options{print_help: true, input_filename: String::from("input_filename"), ..Options::default()},
        },

        //Negative scenarios
        Scn{
            args: vec!["-i"],
            parse_ok: false,
            options: Options{input_filename: String::from("input_filename"), ..Options::default()},
        },
    ];

    for scn in scns.iter() {
        let args: Args = scn.args.iter().map(|s|{s.to_string()}).collect();

        let mut options = Options::new();
        let ok = options.parse(args).is_ok();
        assert_eq!(ok, scn.parse_ok);
        if ok {
            assert_eq!(options, scn.options);
        }
    }
}
