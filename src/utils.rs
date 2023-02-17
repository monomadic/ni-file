use std::{error::Error, path::PathBuf};

#[allow(dead_code)]
pub(crate) fn setup_logger() {
    log::set_logger(&loggy::Loggy {
        prefix: "",
        show_time: false,
        show_thread: true,
    })
    .unwrap();
    log::set_max_level(log::LevelFilter::Debug); // Or whatever level you want.
}

#[allow(dead_code)]
pub(crate) fn get_test_files() -> Result<Vec<PathBuf>, Box<dyn Error>> {
    Ok(glob::glob("data/files/**/*.*")?
        .filter_map(|path| path.ok())
        .filter(|path| path.file_name().unwrap() != ".DS_Store")
        .collect())
}
