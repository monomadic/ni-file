use std::{error::Error, path::PathBuf};

#[allow(dead_code)]
pub(crate) fn get_test_files(path: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let files: Vec<PathBuf> = glob::glob(path)?
        .filter_map(|path| path.ok())
        .filter(|path| path.is_file())
        .filter(|path| path.file_name().unwrap() != ".DS_Store")
        .collect();

    if files.len() > 0 {
        return Ok(files);
    } else {
        panic!("no test files found at {}", path);
    }
}
