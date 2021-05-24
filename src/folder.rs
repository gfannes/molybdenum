use crate::res::Result;
use crate::cli::Options;
use std::path::Path;
use regex::bytes::Regex;

pub struct Scanner<'a> {
    options: &'a Options,
    file_include_patterns: Vec<Regex>,
}

pub type Paths = Vec<std::path::PathBuf>;

impl Scanner<'_> {
    pub fn new<'a>(options: &'a Options) -> Result<Scanner<'a>>
    {
        let scanner = Scanner{
            options,
            file_include_patterns: vec![],
        };
        Ok(scanner)
    }

    pub fn scan(&self) -> Result<Paths> {
        let mut paths = Paths::new();
        self.scan_(&self.options.root_folder, &mut paths)?;
        Ok(paths)
    }

    fn scan_<P>(&self, parent: P, mut paths: &mut Paths) -> Result<()>
        where P: AsRef<Path>
    {
        for entry in std::fs::read_dir(parent.as_ref())? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let path = entry.path();
            let is_hidden = my_is_hidden(&path).unwrap_or(false);

            if file_type.is_file() {
                if is_hidden && !self.options.search_hidden_files {
                    continue;
                }

                if !self.options.extensions.is_empty() {
                    let mut allowed = false;
                    if let Some(extension) = path.extension() {
                        if let Some(extension_str) = extension.to_str() {
                            for allowed_extension in self.options.extensions.iter() {
                                if allowed_extension == extension_str {
                                    allowed = true;
                                    break;
                                }
                            }
                        }
                    }
                    if !allowed {
                        continue;
                    }
                }

                if !self.file_include_patterns.is_empty() {
                    let mut allowed = false;
                    if !allowed {
                        continue;
                    }
                }

                if self.options.use_relative_paths {
                    //strip_prefix() is used to make the paths relative from the specified root
                    //folder
                    paths.push(path.strip_prefix(&self.options.root_folder)?.to_path_buf());
                } else {
                    paths.push(path.to_path_buf());
                }
            } else if file_type.is_dir() {
                if !is_hidden || self.options.search_hidden_folders {
                    self.scan_(path, &mut paths)?
                }
            }
            else if file_type.is_symlink() {
                //Symlinks are skipped for now
            }
        }
        Ok(())
    }
}

fn my_is_hidden<P>(path: P) -> Option<bool>
where P: AsRef<Path>
{
    let ch = path.as_ref().file_name()?.to_str()?.chars().next()?;
    Some(ch == '.')
}

#[test]
fn test_scan_folder() -> Result<()> {
    let options = Options::new();
    let scanner = Scanner::new(&options);
    let paths = scanner.scan()?;
    assert!(paths.len() > 0);

    Ok(())
}
