use std::{error::Error, fs, io::Read};

use ni_file::frame::Frame;

mod utils;

#[test]
fn test_reading_files() -> Result<(), Box<dyn Error>> {
    utils::setup_logger();

    for path in utils::get_test_files()? {
        log::info!("reading {:?}", path);

        let mut file = fs::File::open(path.as_path())?;
        let _frame = Frame::read(&file)?;

        // assure no space left at end of file
        let mut buf = Vec::new();
        file.read(&mut buf)?;
        assert_eq!(buf.len(), 0);
    }

    Ok(())
}
