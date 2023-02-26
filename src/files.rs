use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::logger::Logger;

pub struct Files<'a> {
    pub config_path: PathBuf,
    pub logger: &'a Logger,
}

impl Files<'_> {
    pub fn list(&self) -> Result<(), ()> {
        let file_list = self.get()?;
        for file in file_list {
            self.logger.log(&format!("{}", file.display()))
        }

        Ok(())
    }

    pub fn get(&self) -> Result<Vec<PathBuf>, ()> {
        let config_string = fs::read_to_string(&self.config_path).unwrap_or_else(|_| {
            panic!(
                "Can not read config file: {}",
                self.config_path.to_str().unwrap()
            )
        });

        let files = config_string.lines().map(PathBuf::from).collect();

        Ok(files)
    }

    // Should be used when creating the backup archive
    pub fn get_only_existing(&self) -> Result<Vec<PathBuf>, ()> {
        let files = self.get()?;
        let mut existing_files = files
            .into_iter()
            .filter_map(|maybe_file| {
                if Path::new(&maybe_file).exists() {
                    Some(PathBuf::from(&maybe_file))
                } else {
                    None
                }
            })
            .collect::<Vec<PathBuf>>();

        existing_files.sort();
        existing_files.dedup();
        Ok(existing_files)
    }

    pub fn add(&self, path: &Path) -> Result<(), ()> {
        self.logger.log(&format!("Adding file {}", path.display()));
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.config_path)
            .unwrap();

        writeln!(file, "{}", std::fs::canonicalize(path).unwrap().display()).unwrap();

        Ok(())
    }

    pub fn remove(&self, path: &PathBuf) -> Result<(), ()> {
        self.logger
            .log(&format!("Removing file {}", path.display()));
        let files = self.get()?;
        let files_without: Vec<PathBuf> = files
            .into_iter()
            .filter(|f| f != &std::fs::canonicalize(path).unwrap())
            .collect();
        self.write_file(files_without)
    }

    pub fn clean(&self) -> Result<(), ()> {
        // We do not use get_only_existing here to preserve files which might
        // get created later
        let mut files = self.get()?;
        files.sort();
        files.dedup();
        self.write_file(files)
    }

    fn write_file(&self, files: Vec<PathBuf>) -> Result<(), ()> {
        let mut file = File::create(&self.config_path).unwrap();

        for f in files {
            writeln!(file, "{}", f.display()).unwrap();
        }

        Ok(())
    }
}
