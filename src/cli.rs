use crate::res::{Result,MyError};
use std::collections::VecDeque;

//Represents parsed CLI options
#[derive(Debug,PartialEq,Eq)]
pub struct Options {
    pub print_help: bool,
    pub root_folder: String,
    pub verbose_level: i32,
    pub use_relative_paths: bool,
    pub output_filenames_only: bool,
    pub search_hidden_files: bool,
    pub search_hidden_folders: bool,
}

//Creates a Vec of CLI option handlers
fn generate_option_vec() -> Vec<Option> {
    let mut v = Vec::new();

    v.push(Option::new("-h", "--help", "Print this help", Handler::Args0(|options|{
        options.print_help = true;
        Ok(())
    })));

    v.push(Option::new("-C", "--root-folder", "Root folder", Handler::Args1(|options, filename|{
        options.root_folder = filename.to_string();
        Ok(())
    })));

    v.push(Option::new("-V", "--verbose-level", "Verbosity level", Handler::Args1(|options, level|{
        match level.parse::<i32>() {
            Err(_) => fail!("Could not convert \"{}\" into a verbosity level", level),
            Ok(v) => options.verbose_level = v,
        }
        Ok(())
    })));

    v.push(Option::new("-R", "--relative-paths", "Use paths relative to the respective root folder", Handler::Args0(|options|{
        options.use_relative_paths = true;
        Ok(())
    })));

    v.push(Option::new("-l", "--filenames-only", "Output only filenames", Handler::Args0(|options|{
        options.output_filenames_only = true;
        Ok(())
    })));

    v.push(Option::new("-u", "--hidden-files", "Search hidden files as well", Handler::Args0(|options|{
        options.search_hidden_files = true;
        Ok(())
    })));

    v.push(Option::new("-U", "--hidden-folders", "Search hidden folders as well", Handler::Args0(|options|{
        options.search_hidden_folders = true;
        Ok(())
    })));

    v
}

//Represents raw CLI arguments as provided by the user
pub type Args = VecDeque<String>;

pub fn args() -> Args {
    std::env::args().skip(1).collect()
}

impl Default for Options {
    fn default() -> Options {
        Options{
            print_help: false,
            root_folder: String::from("."),
            verbose_level: 0,
            use_relative_paths: false,
            output_filenames_only: false,
            search_hidden_files: false,
            search_hidden_folders: false,
        }
    }
}

impl Options {
    pub fn new() -> Options {
        Options::default()
    }

    pub fn parse(&mut self, mut args: Args) -> Result<()> {
        let options = generate_option_vec();

        //Process all CLI arguments
        while let Some(arg0) = args.pop_front() {

            //Find option that matches with arg0
            match options.iter().find(|option|{option.suit(&arg0)}) {
                None => fail!("Unknown option \"{}\"", &arg0),

                //Call the handler, taking care of its amount of arguments
                Some(option) => match option.handler {
                    Handler::Args0(ftor) => ftor(self)?,

                    Handler::Args1(ftor) => match args.pop_front() {
                        None => fail!("Option {} expects additional argument", option.lh),

                        Some(arg1) => ftor(self, &arg1)?,
                    },
                },
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
    #[derive(Debug)]
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
            args: vec!["-C", "root_folder"],
            parse_ok: true,
            options: Options{root_folder: String::from("root_folder"), ..Options::default()},
        },
        Scn{
            args: vec!["-V", "3"],
            parse_ok: true,
            options: Options{verbose_level: 3, ..Options::default()},
        },
        //All options
        Scn{
            args: vec!["-h", "-C", "root_folder"],
            parse_ok: true,
            options: Options{print_help: true, root_folder: String::from("root_folder"), ..Options::default()},
        },

        //Negative scenarios
        Scn{
            args: vec!["--unknown-option"],
            parse_ok: false,
            options: Options{..Options::default()},
        },
        Scn{
            args: vec!["-C"],
            parse_ok: false,
            options: Options{root_folder: String::from("root_folder"), ..Options::default()},
        },
        Scn{
            args: vec!["-C", "-h"],
            parse_ok: true,
            options: Options{print_help: false, root_folder: String::from("-h"), ..Options::default()},
        },
        ];

    for scn in scns.iter() {
        println!("{:?}", scn);

        let args: Args = scn.args.iter().map(|s|{s.to_string()}).collect();

        let mut options = Options::new();
        let ok = options.parse(args).is_ok();
        assert_eq!(ok, scn.parse_ok);
        if ok {
            assert_eq!(options, scn.options);
        }
    }
}
