mod utils;

use ni_file::{nis::AuthoringApplication, Repository};
use std::fs;

#[test]
fn test_reading_nisound_kontakt() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: .nkm support
    for path in utils::get_test_files("tests/data/nisound/file/kontakt/**/*.nki")? {
        println!("reading {:?}", path);

        let file = fs::File::open(path.as_path())?;
        let sound = Repository::read(&file)?;

        assert_eq!(
            sound.authoring_application()?,
            AuthoringApplication::Kontakt
        );
    }

    Ok(())
}
