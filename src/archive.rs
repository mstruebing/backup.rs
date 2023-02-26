use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::path::PathBuf;

use crate::logger::Logger;

// Creates a gzipped tar file of given files in a given destination
pub fn create(files: Vec<PathBuf>, destination: PathBuf, logger: &Logger) -> Result<(), ()> {
    logger.verbose(&format!("Creating tar file: {}", destination.display()));
    let tar_gz = File::create(destination).unwrap();
    let enc = GzEncoder::new(tar_gz, Compression::best());
    let mut tar = tar::Builder::new(enc);

    // prevent from panic in case of broken symlinks
    tar.follow_symlinks(false);

    for file in files {
        logger.debug(&format!("Add file to archive: {}", file.display()));
        let filename_for_tar = file.strip_prefix("/").unwrap();

        if file.is_dir() {
            tar.append_dir_all(filename_for_tar, file.clone()).unwrap()
        } else if file.is_file() {
            tar.append_path_with_name(file.clone(), filename_for_tar)
                .unwrap();
        }
    }

    Ok(())
}
