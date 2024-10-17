use crate::cli::{Options, OutputOnly};
use crate::util::{MyError, Result};
use ignore::WalkBuilder;
use regex::bytes::{Regex, RegexBuilder};
use std::collections::BTreeSet;
use std::ffi::OsString;
use std::path::Path;

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
    where
        P: AsRef<std::path::Path>,
    {
        let mut scanner = Scanner {
            root: std::path::PathBuf::from(root.as_ref()),
            options,
            file_include_regex_vec: vec![],
            file_exclude_regex_vec: vec![],
            binary_extensions: all_binary_extensions_(),
        };
        for s in options.file_include_pattern_vec.iter() {
            match RegexBuilder::new(s)
                .case_insensitive(!options.case_sensitive)
                .build()
            {
                Err(_) => fail!("'{}' is not a valid Regex", s),
                Ok(re) => scanner.file_include_regex_vec.push(re),
            }
        }
        for s in options.file_exclude_pattern_vec.iter() {
            match RegexBuilder::new(s)
                .case_insensitive(!options.case_sensitive)
                .build()
            {
                Err(_) => fail!("'{}' is not a valid Regex", s),
                Ok(re) => scanner.file_exclude_regex_vec.push(re),
            }
        }
        Ok(scanner)
    }

    pub fn scan(&self) -> Result<Paths> {
        let mut paths = Paths::new();
        self.walk_(&self.root, &mut paths)?;
        Ok(paths)
    }

    fn walk_<P>(&self, parent: P, paths: &mut Paths) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let walk = WalkBuilder::new(parent)
            .hidden(!self.options.search_hidden_files)
            .ignore(!self.options.search_ignored_files)
            .git_ignore(!self.options.search_ignored_files)
            .git_exclude(!self.options.search_ignored_files)
            .git_global(!self.options.search_ignored_files)
            .build();

        for entry in walk {
            match entry {
                Err(err) => {
                    if self.options.verbose_level >= 1 {
                        println!("Warning: could not walk this entry: {:?}", err);
                    }
                }
                Ok(entry) => {
                    let file_type = match entry.file_type() {
                        None => fail!("Could not get file type for '{:?}'", entry),
                        Some(ft) => ft,
                    };
                    let path = entry.into_path();

                    let do_add_path = match self.options.output_only {
                        None | Some(OutputOnly::Match) => {
                            true && file_type.is_file()
                                && self.extension_ok_(&path)
                                && self.name_ok_(&path)
                        }

                        Some(OutputOnly::Filenames) => {
                            true && file_type.is_file()
                                && self.extension_ok_(&path)
                                && self.name_ok_(&path)
                        }

                        Some(OutputOnly::Folders) => {
                            true && file_type.is_dir() && self.name_ok_(&path)
                        }
                    };

                    if do_add_path {
                        paths.push(path.to_path_buf());
                    }
                }
            }
        }
        Ok(())
    }

    fn extension_ok_(&self, path: &std::path::Path) -> bool {
        //Filter against allowed extensions, if any
        if let Some(extension) = path.extension() {
            let is_binary = self.binary_extensions.contains(extension);
            if is_binary && !self.options.search_binary_files {
                return false;
            }

            if !self.options.extensions.is_empty() {
                let mut extension_dot = OsString::from(".");
                extension_dot.push(extension);
                if !self.options.extensions.iter().any(|allowed_extension| {
                    allowed_extension == extension || allowed_extension == extension_dot.as_os_str()
                }) {
                    return false;
                }
            }
        } else {
            if !self.options.extensions.is_empty() {
                return false;
            }
        }
        true
    }
    fn name_ok_(&self, path: &std::path::Path) -> bool {
        //Filter against include/exclude patterns
        match path.to_str() {
            None => println!(
                "Warning: path '{}' is not UTF-8 and cannot be matched",
                path.display()
            ),

            Some(path_str) => {
                if !self
                    .file_include_regex_vec
                    .iter()
                    .all(|re| re.is_match(path_str.as_bytes()))
                {
                    return false;
                }
                if self
                    .file_exclude_regex_vec
                    .iter()
                    .any(|re| re.is_match(path_str.as_bytes()))
                {
                    return false;
                }
            }
        }
        true
    }
}

fn all_binary_extensions_() -> BTreeSet<OsString> {
    let mut set = BTreeSet::<OsString>::new();
    for ext in &[
        "wav", "rlib", "rmeta", "dat", "bin", "exe", "png", "jpg", "jpeg", "pdf", "so", "a", "pyc",
        "zip", "gz", "gzip", "o", "out", "dll",
    ] {
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
