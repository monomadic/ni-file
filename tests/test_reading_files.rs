mod utils;

use ni_file::NIRepository;
use std::{fs, io::Read};

#[test]
fn test_reading_test_files() -> Result<(), Box<dyn std::error::Error>> {
    utils::setup_logger();

    for path in utils::get_test_files()? {
        log::info!("reading {:?}", path);

        let mut file = fs::File::open(path.as_path())?;
        let _frame = NIRepository::read(&file)?;

        // assure no space left at end of file
        let mut buf = Vec::new();
        file.read(&mut buf)?;
        assert_eq!(buf.len(), 0);
    }

    Ok(())
}
