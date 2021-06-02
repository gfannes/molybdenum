use crate::util::{Result,MyError};
use crate::cli::Options;
use std::path::Path;
use std::ffi::OsString;
use std::collections::BTreeSet;
use regex::bytes::Regex;
use ignore::WalkBuilder;

pub struct Scanner<'a> {
    root: std::path::PathBuf,
    options: &'a Options,
    file_include_regex_vec: Vec<Regex>,
    file_exclude_regex_vec: Vec<Regex>,
    binary_extensions: BTreeSet<OsString>,
}

pub type Paths = Vec<std::path::PathBuf>;

impl Scanner<'_> {
    pub fn new<'a, P>(root: P, options: &'a Options) -> Result<Scanner<'a>>
        where P: AsRef<std::path::Path>
    {
        let mut scanner = Scanner{
            root: std::path::PathBuf::from(root.as_ref()),
            options,
            file_include_regex_vec: vec![],
            file_exclude_regex_vec: vec![],
            binary_extensions: all_binary_extensions_(),
        };
        for s in options.file_include_pattern_vec.iter() {
            match Regex::new(s) {
                Err(_) => fail!("\"{}\" is not a valid Regex", s),
                Ok(re) => scanner.file_include_regex_vec.push(re),
            }
        }
        for s in options.file_exclude_pattern_vec.iter() {
            match Regex::new(s) {
                Err(_) => fail!("\"{}\" is not a valid Regex", s),
                Ok(re) => scanner.file_exclude_regex_vec.push(re),
            }
        }
        Ok(scanner)
    }

    pub fn scan(&self) -> Result<Paths> {
        let mut paths = Paths::new();
        self.walk_(&self.root, &mut paths);
        Ok(paths)
    }

    fn walk_<P>(&self, parent: P, mut paths: &mut Paths) -> Result<()>
        where P: AsRef<Path>
    {
        let walk = WalkBuilder::new(parent)
            .hidden(!self.options.search_hidden_files)
            .ignore(!self.options.search_ignored_files)
            .git_ignore(!self.options.search_ignored_files)
            .git_exclude(!self.options.search_ignored_files)
            .git_global(!self.options.search_ignored_files)
            .build();

        for entry in walk {
            let entry = entry?;
            let file_type = match entry.file_type() {
                None => fail!("Could not get file type for \"{:?}\"", entry),
                Some(ft) => ft,
            };
            let path = entry.into_path();

            if file_type.is_file() {
                //Filter against allowed extensions, if any
                if let Some(extension) = path.extension() {
                    let is_binary = self.binary_extensions.contains(extension);
                    if is_binary && !self.options.search_binary_files {
                        continue;
                    }

                    if !self.options.extensions.is_empty() {
                        let mut extension_dot = OsString::from(".");
                        extension_dot.push(extension);
                        if !self.options.extensions.iter().any(|allowed_extension| allowed_extension == extension || allowed_extension == extension_dot.as_os_str()) {
                            continue;
                        }
                    }
                } else {
                    if !self.options.extensions.is_empty() {
                        continue;
                    }
                }

                //Filter against include/exclude patterns
                match path.to_str() {
                    None => println!("Warning: path \"{}\" is not UTF-8 and cannot be matched", path.display()),

                    Some(path_str) => {
                        if !self.file_include_regex_vec.iter().all(|re|{re.is_match(path_str.as_bytes())}) {
                            continue;
                        }
                        if self.file_exclude_regex_vec.iter().any(|re|{re.is_match(path_str.as_bytes())}) {
                            continue;
                        }
                    },
                }

                if self.options.use_relative_paths {
                    //strip_prefix() is used to make the paths relative from the specified root
                    //folder
                    paths.push(path.strip_prefix(&self.root)?.to_path_buf());
                } else {
                    paths.push(path.to_path_buf());
                }
            }
        }
        Ok(())
    }
}

fn all_binary_extensions_() -> BTreeSet<OsString> {
    let mut set = BTreeSet::<OsString>::new();
    for ext in &["wav", "rlib", "rmeta", "dat", "bin", "exe", "png", "jpg", "jpeg", "pdf", "so", "a", "pyc", "zip", "gz", "gzip", "o"] {
        set.insert(OsString::from(ext));
    }
    set
}

#[test]
fn test_scan_folder() -> Result<()> {
    let options = Options::new();
    let scanner = Scanner::new(".", &options)?;
    let paths = scanner.scan()?;
    assert!(paths.len() > 0);

    Ok(())
}
