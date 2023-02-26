use std::path::PathBuf;

use crate::logger::Logger;

#[derive(Debug)]
pub struct Config {
    pub files: PathBuf,
    pub cache: PathBuf,
    pub remotes: PathBuf,
}

impl Config {
    pub fn new(prefix: &str, logger: &Logger) -> Config {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(prefix).unwrap();

        let files = xdg_dirs
            .place_config_file("files")
            .expect("cannot create configuration directory");

        let remotes = xdg_dirs
            .place_config_file("remotes")
            .expect("cannot create configuration directory");

        let cache = xdg_dirs
            .place_cache_file("archive.tar.gz")
            .expect("cannot create cache directory");

        logger.debug(&format!("Creating file if not exists: {}", files.display()));
        create_file(files.to_owned()).ok();

        logger.debug(&format!(
            "Creating file if not exists: {}",
            remotes.display()
        ));
        create_file(remotes.to_owned()).ok();

        logger.debug(&format!("Creating file if not exists: {}", cache.display()));
        create_file(cache.to_owned()).ok();

        Config {
            files,
            remotes,
            cache,
        }
    }
}

fn create_file(path: PathBuf) -> Result<(), ()> {
    let directory = path.parent();

    match directory {
        Some(d) => {
            if !d.is_dir() {
                std::fs::create_dir_all(d).unwrap();
            }
        }
        None => return Err(()),
    };

    // Create files file if it doesn't exist
    if !path.is_file() {
        std::fs::File::create(path).unwrap();
    }

    Ok(())
}
