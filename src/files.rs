use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

// TODO: Remove files which are already in a directory???
//  TODO: Tests
// TODO: Own Errors

pub struct Files {
    pub config_path: PathBuf,
}

impl Files {
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

    // TODO: Write a function which is capable of receiving a vector
    // to reduce writes
    pub fn add(&self, path: &Path) -> Result<(), ()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.config_path)
            .unwrap();

        writeln!(file, "{}", path.display()).unwrap();

        Ok(())
    }

    pub fn remove(&self, path: &PathBuf) -> Result<(), ()> {
        let files = self.get()?;
        let files_without: Vec<PathBuf> = files.into_iter().filter(|f| f != path).collect();
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

#[cfg(test)]
mod tests {
    // use super::Files;
    // use std::env;
    // use std::fs::File;
    // use std::path::PathBuf;

    // fn init() -> String {
    //     let mut path = env::temp_dir();
    //     path.push("files");
    //     File::create(&path).unwrap();
    //     path.to_str().unwrap().to_owned()
    // }

    // #[test]
    // fn test_add() {
    //     let path = init();
    //     let files = Files::new(&path);
    //     assert_eq!(files.get().unwrap(), Vec::<PathBuf>::new());

    //     files.add(&PathBuf::from("blubber")).unwrap();
    //     assert_eq!(files.get().unwrap(), vec![PathBuf::from("blubber")]);

    //     files.add(&PathBuf::from("bam")).unwrap();
    //     assert_eq!(
    //         files.get().unwrap(),
    //         vec![PathBuf::from("blubber"), PathBuf::from("bam")]
    //     );
    // }
}
