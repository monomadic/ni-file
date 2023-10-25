mod utils;

use ni_file::nis::{AuthoringApplication, Repository};
use std::fs;

#[test]
#[ignore]
fn test_nis_kontakt_custom_dir() -> Result<(), Box<dyn std::error::Error>> {
    for path in utils::get_test_files("../ni-file-data/NISound/**/Kontakt/**/*.*")? {
        dbg!(&path);
        let file = fs::File::open(path.as_path())?;
        let nis = Repository::read(&file)?;

        assert_eq!(nis.authoring_application()?, AuthoringApplication::Kontakt);
    }

    Ok(())
}

#[test]
#[ignore]
fn test_nis_read_all() -> Result<(), Box<dyn std::error::Error>> {
    for path in utils::get_test_files("../ni-file-data/NISound/**/*.*")? {
        dbg!(&path);
        let file = fs::File::open(path.as_path())?;
        let repo = Repository::read(&file)?;

        let root = repo.find_repository_root().unwrap()?;
        assert_eq!(root.repository_magic, 0);
        assert_eq!(root.repository_type, 1);

        match repo.detect() {
            ni_file::nis::RepositoryType::KontaktPreset => {
                let _preset = repo.item().extract_kontakt_preset().unwrap()?;
            }
            ni_file::nis::RepositoryType::AppSpecific => todo!(),
            ni_file::nis::RepositoryType::Preset => todo!(),
            ni_file::nis::RepositoryType::Unknown => todo!(),
        }
    }

    Ok(())
}
