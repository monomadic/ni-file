mod utils;

use ni_file::{nis::AuthoringApplication, Repository};
use std::fs;

// #[test]
// fn test_reading_nisound_kontakt() -> Result<(), Box<dyn std::error::Error>> {
//     // TODO: .nkm support
//     for path in utils::get_test_files("tests/data/nisound/file/kontakt/**/*.nki")? {
//         println!("reading {:?}", path);
//
//         let file = fs::File::open(path.as_path())?;
//         let sound = Repository::read(&file)?;
//
//         assert_eq!(
//             sound.authoring_application()?,
//             AuthoringApplication::Kontakt
//         );
//     }
//
//     Ok(())
// }

#[test]
#[ignore]
fn test_nis_kontakt_custom_dir() -> Result<(), Box<dyn std::error::Error>> {
    for path in utils::get_test_files("../ni-file-data/kontakt/nis/**/*.*")? {
        dbg!(&path);
        let file = fs::File::open(path.as_path())?;
        let nis = Repository::read(&file)?;

        assert_eq!(nis.authoring_application()?, AuthoringApplication::Kontakt);

        dbg!(&nis.preset()?);
    }

    Ok(())
}

#[test]
#[ignore]
fn test_nis_read_all() -> Result<(), Box<dyn std::error::Error>> {
    for path in utils::get_test_files("../ni-file-data/**/nis/**/*.*")? {
        dbg!(&path);
        let file = fs::File::open(path.as_path())?;
        let repo = Repository::read(&file)?;

        // assert_eq!(nis.authoring_application()?, AuthoringApplication::Kontakt);

        let root = repo.root()?;
        assert_eq!(root.repository_magic, 0);
        assert_eq!(root.repository_type, 1);

        dbg!(&repo.preset()?);
    }

    Ok(())
}
