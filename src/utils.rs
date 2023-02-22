use crate::prelude::*;
use std::path::PathBuf;

#[allow(dead_code)]
pub(crate) fn setup_logger() {
    let _ = log::set_logger(&loggy::Loggy {
        prefix: "",
        show_time: false,
        show_thread: true,
    });
    log::set_max_level(log::LevelFilter::Debug); // Or whatever level you want.
}

#[allow(dead_code)]
pub(crate) fn get_test_files() -> Result<Vec<PathBuf>> {
    let path = "../tests/data/files/**/*.*";
    Ok(glob::glob(path)
        .map_err(|_| NIFileError::Generic(format!("error globbing: {}", path)))?
        .filter_map(|path| path.ok())
        .filter(|path| path.file_name().unwrap() != ".DS_Store")
        .collect())
}

#[allow(dead_code)]
pub(crate) fn glob_paths(path: &str) -> Result<Vec<PathBuf>> {
    Ok(glob::glob(path)
        .map_err(|_| NIFileError::Generic(format!("error globbing: {}", path)))?
        .filter_map(|path| path.ok())
        .filter(|path| path.file_name().unwrap() != ".DS_Store")
        .collect())
}

#[allow(dead_code)]
fn format_hex(buffer: &[u8]) -> String {
    format!(
        "{}",
        &buffer
            .iter()
            .map(|x| format!("{:02x} ", x))
            .collect::<String>()
    )
}

#[allow(dead_code)]
fn format_ascii(buffer: &[u8]) -> String {
    format!("{}", String::from_utf8_lossy(buffer).to_string())
    // format!("{}", &buffer.iter().map(|x| if x.is_ascii() {'s'} else {' '})).collect::<String>())
}
