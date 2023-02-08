use std::{error::Error, fs};

use ni_file::frame::Frame;

mod utils;

#[test]
fn test_reading_files() -> Result<(), Box<dyn Error>> {
    for path in utils::get_test_files()? {
        println!("reading {:?}", path);

        let file = fs::File::open(path.as_path())?;
        let frame = Frame::read(file);

        assert!(frame.is_ok());
        assert_eq!(frame?.len(), 0);
    }

    Ok(())
}
