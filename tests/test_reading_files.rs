mod utils;

use ni_file::{NIContainer, NIMonolith};
use std::{fs, io::Read};

#[test]
fn test_reading_ni_containers() -> Result<(), Box<dyn std::error::Error>> {
    utils::setup_logger();

    for path in utils::get_test_files("tests/files/nicontainer/file/**/*.*")? {
        log::info!("reading {:?}", path);

        let mut file = fs::File::open(path.as_path())?;
        let _frame = NIContainer::read(&file)?;

        // assure no space left at end of file
        let mut buf = Vec::new();
        file.read(&mut buf)?;
        assert_eq!(buf.len(), 0);
    }

    Ok(())
}

#[test]
fn test_reading_ni_monolith() -> Result<(), Box<dyn std::error::Error>> {
    for path in utils::get_test_files("tests/data/monolith/**/*.*")? {
        log::info!("reading {:?}", path);

        let file = fs::File::open(path.as_path())?;
        let _frame = NIMonolith::read(file)?;
    }

    Ok(())
}
