mod utils;

use ni_file::NIMonolith;
use std::fs;

#[test]
fn test_reading_ni_monolith() -> Result<(), Box<dyn std::error::Error>> {
    for path in utils::get_test_files("tests/data/monolith/**/*.*")? {
        log::info!("reading {:?}", path);

        let file = fs::File::open(path.as_path())?;
        let _frame = NIMonolith::read(file)?;
    }

    Ok(())
}
