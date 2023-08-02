mod utils;

use ni_file::nks::nksfile::NKSFile;
use std::fs;

#[test]
fn test_reading_nks() -> Result<(), Box<dyn std::error::Error>> {
    for path in utils::get_test_files("tests/filetypes/nks/4.*/*.*")? {
        log::info!("\nReading {:?}", path);

        let file = fs::File::open(path.as_path())?;
        let _frame = NKSFile::read(file)?;
    }

    Ok(())
}
