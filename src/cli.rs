use crate::util::{MyError, Result};
use colored::Colorize;
use std::collections::VecDeque;
use std::ffi::OsString;

//<Specific part of CLI handling>
//
#[derive(Debug, PartialEq, Eq)]
pub enum OutputOnly {
    Filenames,
    Folders,
    Match,
}
//
//Represents parsed CLI options
#[derive(Debug, PartialEq, Eq)]
pub struct Options {
    pub output_help: bool,
    pub roots: Vec<String>,
    pub verbose_level: i32,
    pub output_only: std::option::Option<OutputOnly>,
    pub null_separated_output: bool,
    pub search_hidden_files: bool,
    pub search_ignored_files: bool,
    pub search_pattern_opt: std::option::Option<String>,
    pub capture_group_prefix_opt: std::option::Option<String>,
    pub invert_pattern: bool,
    pub replace_opt: std::option::Option<String>,
    pub simulate_replace: bool,
    pub word_boundary: bool,
    pub case_sensitive: bool,
    pub extensions: Vec<OsString>,
    pub extension_sets: Vec<OsString>,
    pub search_binary_files: bool,
    pub file_include_pattern_vec: Vec<String>,
    pub file_exclude_pattern_vec: Vec<String>,
    pub output_after: u64,
    pub output_before: u64,
    pub input_from_file_opt: std::option::Option<bool>,
    pub console_output: std::option::Option<bool>,
    pub color_output: std::option::Option<bool>,
}

//Default values for Options
impl Default for Options {
    fn default() -> Options {
        Options {
            output_help: false,
            roots: vec![],
            verbose_level: 0,
            output_only: None,
            null_separated_output: false,
            search_hidden_files: false,
            search_ignored_files: false,
            search_pattern_opt: None,
            capture_group_prefix_opt: None,
            invert_pattern: false,
            replace_opt: None,
            simulate_replace: false,
            word_boundary: false,
            case_sensitive: false,
            extensions: vec![],
            extension_sets: vec![],
            search_binary_files: false,
            file_include_pattern_vec: vec![],
            file_exclude_pattern_vec: vec![],
            output_after: 0,
            output_before: 0,
            input_from_file_opt: None,
            console_output: None,
            color_output: None,
        }
    }
}

fn parse_boolean(s: &str) -> bool {
    match s {
        "true" | "1" | "y" | "Y" | "yes" | "Yes" | "YES" => true,
        _ => false,
    }
}

//Creates a Vec of CLI option handlers
fn generate_option_vec() -> Vec<Option> {
    vec![
        Option::new("-h", "--help", "Print this help [false]", Handler::Args0(|options|{
            options.output_help = true;
            Ok(())
        })),
        Option::new("-C", "--root", "Add FOLDER as root search folder", Handler::Args1("FOLDER", |options, folder|{
            options.roots.push(folder.to_string());
            Ok(())
        })),
        Option::new("-V", "--verbose", "Use verbosity LEVEL [0]", Handler::Args1("LEVEL", |options, level|{
            match level.parse::<i32>() {
                Err(_) => fail!("Could not convert '{}' into a verbosity level", level),
                Ok(v) => options.verbose_level = v,
            }
            Ok(())
        })),
        Option::new("-l", "--filenames-only", "Output only filenames [false]", Handler::Args0(|options|{
            options.output_only = Some(OutputOnly::Filenames);
            Ok(())
        })),
        Option::new("-L", "--folders-only", "Output only folders [false]", Handler::Args0(|options|{
            options.output_only = Some(OutputOnly::Folders);
            Ok(())
        })),
        Option::new("-m", "--match-only", "Output only matches [false]", Handler::Args0(|options|{
            options.output_only = Some(OutputOnly::Match);
            Ok(())
        })),
        Option::new("-0", "--null", "NULL-separated filename output [false]", Handler::Args0(|options|{
            options.null_separated_output = true;
            Ok(())
        })),
        Option::new("-u", "--hidden-files", "Search hidden files as well [false]", Handler::Args0(|options|{
            options.search_hidden_files = true;
            Ok(())
        })),
        Option::new("-U", "--ignored-files", "Search ignored files as well [false]", Handler::Args0(|options|{
            options.search_ignored_files = true;
            Ok(())
        })),
        Option::new("-p", "--pattern", "Use search regex PATTERN", Handler::Args1("PATTERN", |options, pattern|{
            options.set_search_pattern(pattern);
            Ok(())
        })),
        Option::new("-P", "--capture-prefix", "Substitute capture groups with given prefix. If no capture group index is provided, 1 will be used.", Handler::Args1("PREFIX", |options, prefix|{
            options.capture_group_prefix_opt = Some(prefix.to_string());
            Ok(())
        })),
        Option::new("-v", "--invert", "Invert search pattern", Handler::Args0(|options|{
            options.invert_pattern = true;
            Ok(())
        })),
        Option::new("-r", "--replace", "Replace search matches with STRING", Handler::Args1("STRING", |options, replace|{
            options.replace_opt = Some(replace.to_string());
            Ok(())
        })),
        Option::new("-n", "--simulate", "Simulate replacement without writing output", Handler::Args0(|options|{
            options.simulate_replace = true;
            Ok(())
        })),
        Option::new("-w", "--word", "Search for word boundary [false]", Handler::Args0(|options|{
            options.word_boundary = true;
            Ok(())
        })),
        Option::new("-s", "--sensitive", "Case-sensitive search [false]", Handler::Args0(|options|{
            options.case_sensitive = true;
            Ok(())
        })),
        Option::new("-e", "--extension", "Add search EXTENSION (or)", Handler::Args1("EXTENSION", |options, extension|{
            options.extensions.push(OsString::from(extension));
            Ok(())
        })),
        Option::new("-E", "--extension-set", "Add search EXTENSION_SET (or)", Handler::Args1("EXTENSION_SET", |options, extenion_set|{
            options.extension_sets.push(OsString::from(extenion_set));
            Ok(())
        })),
        Option::new("-a", "--binary", "Search binary files as well [false]", Handler::Args0(|options|{
            options.search_binary_files = true;
            Ok(())
        })),
        Option::new("-f", "--include-filepath", "Add PATTERN to select files (and)", Handler::Args1("PATTERN", |options, pattern|{
            options.file_include_pattern_vec.push(pattern.to_string());
            Ok(())
        })),
        Option::new("-F", "--exclude-filepath", "Add PATTERN to exclude files (or)", Handler::Args1("PATTERN", |options, pattern|{
            options.file_exclude_pattern_vec.push(pattern.to_string());
            Ok(())
        })),
        Option::new("-A", "--output-after", "Output NUMBER lines after each match [0]", Handler::Args1("NUMBER", |options, number|{
            options.output_after = number.parse()?;
            Ok(())
        })),
        Option::new("-B", "--output-before", "Output NUMBER lines before each match [0]", Handler::Args1("NUMBER", |options, number|{
            options.output_before = number.parse()?;
            Ok(())
        })),
        Option::new("-i", "--input-file", "Take input from file, override auto-detection", Handler::Args0(|options|{
            options.input_from_file_opt = Some(true);
            Ok(())
        })),
        Option::new("-I", "--input-stream", "Take input from redirected stream, override auto-detection", Handler::Args0(|options|{
            options.input_from_file_opt = Some(false);
            Ok(())
        })),
        Option::new("-c", "--console-output", "Produce console or compact output [false if output stream is TTY]", Handler::Args1("BOOLEAN", |options, boolean|{
            options.console_output = Some(parse_boolean(boolean));
            Ok(())
        })),
        Option::new("-k", "--color-output", "Produce colored output [false if output stream is TTY]", Handler::Args1("BOOLEAN", |options, boolean|{
            options.color_output = Some(parse_boolean(boolean));
            Ok(())
        })),
        ]
}
//</Specific part of CLI handling>

//<Generic part of CLI handling>
//
//This should be moved to a generic layer, or even better,
//be replaced with a CLI parsing crate from crates.io.
//For now, this is not done because this project is still a "learning project".
//
//Represents raw CLI arguments as provided by the user
pub type Args = VecDeque<String>;

pub fn args() -> Args {
    std::env::args().skip(1).collect()
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
            match options.iter().find(|option| option.suit(&arg0)) {
                None => self.set_search_pattern(&arg0),

                //Call the handler, taking care of its amount of arguments
                Some(option) => match option.handler {
                    Handler::Args0(ftor) => ftor(self)?,

                    Handler::Args1(name, ftor) => match args.pop_front() {
                        None => fail!(
                            "Option {} expects additional argument for {}",
                            option.lh,
                            name
                        ),

                        Some(arg1) => ftor(self, &arg1)?,
                    },
                },
            }
        }

        //Translate extension_sets into extensions
        {
            let extension_sets = self.extension_sets.clone();
            for extension_set in extension_sets.iter() {
                let mut add_extension = |extension| self.extensions.push(OsString::from(extension));
                if extension_set == &OsString::from("c") {
                    add_extension("c");
                    add_extension("h");
                    add_extension("cpp");
                    add_extension("hpp");
                } else {
                    fail!("Unknown extension set {}", extension_set.to_string_lossy())
                }
            }
        }

        Ok(())
    }

    pub fn set_search_pattern(&mut self, pattern: &str) {
        if let Some(old_pattern) = &self.search_pattern_opt {
            println!(
                "Warning: Search PATTERN is already set to '{}', setting it now to '{}'.",
                old_pattern, pattern
            );
        }
        self.search_pattern_opt = Some(pattern.to_string());
    }

    pub fn help(&self) -> String {
        let mut s = String::new();

        s.push_str(&format!(
            "Help for the Molybdenum Replacer: {}:\n",
            "mo --OPTION* PATTERN? --OPTION*".green()
        ));
        s.push_str("Dashed options and search PATTERN can be mixed\n");

        s.push_str(&format!("{}:\n", "Options".yellow()));
        let option_vec = generate_option_vec();
        let max_sh_len = option_vec.iter().map(|o| o.sh.len()).max().unwrap();
        let max_lh_len = option_vec.iter().map(|o| o.lh.len()).max().unwrap();
        let max_name_len = option_vec
            .iter()
            .map(|o| {
                if let Handler::Args1(name, _) = o.handler {
                    name.len()
                } else {
                    0
                }
            })
            .max()
            .unwrap();
        for o in option_vec.iter() {
            s.push_str(&o.help(max_sh_len, max_lh_len, max_name_len));
            s.push_str("\n");
        }

        s.push_str(&format!(
            "Version {}, created by Geert Fannes",
            env!("CARGO_PKG_VERSION")
        ));

        s
    }
}

//Handler for the raw CLI arguments
enum Handler {
    Args0(Handler0),
    Args1(&'static str, Handler1),
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
    fn new(sh: &'static str, lh: &'static str, descr: &'static str, handler: Handler) -> Option {
        let o = Option {
            sh,
            lh,
            descr,
            handler,
        };
        o
    }

    fn help(&self, max_sh_len: usize, max_lh_len: usize, name_len: usize) -> String {
        let name = match self.handler {
            Handler::Args1(name, _) => name,
            _ => "",
        };
        format!(
            "    {:max_sh_len$}|{:max_lh_len$} {:name_len$}    {}",
            self.sh.yellow(),
            self.lh.yellow(),
            name.blue(),
            self.descr,
            max_sh_len = max_sh_len,
            max_lh_len = max_lh_len,
            name_len = name_len
        )
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
        Scn {
            args: vec!["-h"],
            parse_ok: true,
            options: Options {
                output_help: true,
                ..Options::default()
            },
        },
        Scn {
            args: vec!["-v"],
            parse_ok: true,
            options: Options {
                invert_pattern: true,
                ..Options::default()
            },
        },
        Scn {
            args: vec!["-C", "ROOT"],
            parse_ok: true,
            options: Options {
                roots: vec![String::from("ROOT")],
                ..Options::default()
            },
        },
        Scn {
            args: vec!["-C", "FOLDER1", "-C", "FOLDER2"],
            parse_ok: true,
            options: Options {
                roots: vec![String::from("FOLDER1"), String::from("FOLDER2")],
                ..Options::default()
            },
        },
        Scn {
            args: vec!["-V", "3"],
            parse_ok: true,
            options: Options {
                verbose_level: 3,
                ..Options::default()
            },
        },
        Scn {
            args: vec!["PATTERN"],
            parse_ok: true,
            options: Options {
                search_pattern_opt: Some("PATTERN".to_string()),
                ..Options::default()
            },
        },
        Scn {
            args: vec!["PATTERN1", "PATTERN2"],
            parse_ok: true,
            options: Options {
                search_pattern_opt: Some("PATTERN2".to_string()),
                ..Options::default()
            },
        },
        Scn {
            args: vec!["PATTERN1", "-p", "PATTERN2"],
            parse_ok: true,
            options: Options {
                search_pattern_opt: Some("PATTERN2".to_string()),
                ..Options::default()
            },
        },
        Scn {
            args: vec!["-p", "PATTERN1", "-p", "PATTERN2"],
            parse_ok: true,
            options: Options {
                search_pattern_opt: Some("PATTERN2".to_string()),
                ..Options::default()
            },
        },
        Scn {
            args: vec!["PATTERN", "-h"],
            parse_ok: true,
            options: Options {
                output_help: true,
                search_pattern_opt: Some("PATTERN".to_string()),
                ..Options::default()
            },
        },
        //All options
        Scn {
            args: vec!["-h", "-C", "ROOT"],
            parse_ok: true,
            options: Options {
                output_help: true,
                roots: vec![String::from("ROOT")],
                ..Options::default()
            },
        },
        //Negative scenarios
        Scn {
            args: vec!["-C"],
            parse_ok: false,
            options: Options {
                roots: vec![String::from("ROOT")],
                ..Options::default()
            },
        },
        Scn {
            args: vec!["-C", "-h"],
            parse_ok: true,
            options: Options {
                output_help: false,
                roots: vec![String::from("-h")],
                ..Options::default()
            },
        },
    ];

    for scn in scns.iter() {
        println!("{:?}", scn);

        let args: Args = scn.args.iter().map(|s| s.to_string()).collect();

        let mut options = Options::new();
        let ok = options.parse(args).is_ok();
        assert_eq!(ok, scn.parse_ok);
        if ok {
            assert_eq!(options, scn.options);
        }
    }
}
//</Generic part of CLI handling>
