use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::path::PathBuf;

// Creates a gzipped tar file of given files in a given destination with maximum
// compression
pub fn create(files: Vec<PathBuf>, destination: PathBuf) -> Result<(), ()> {
    let tar_gz = File::create(destination).unwrap();
    let enc = GzEncoder::new(tar_gz, Compression::best());
    let mut tar = tar::Builder::new(enc);

    // prevent from panic in case of broken symlinks
    tar.follow_symlinks(false);

    for file in files {
        if file.is_dir() {
            tar.append_dir_all(file.strip_prefix("/").unwrap(), file.clone())
                .unwrap()
        } else if file.is_file() {
            tar.append_path_with_name(file.clone(), file).unwrap();
        }
    }

    Ok(())
}
