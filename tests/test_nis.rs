mod utils;

use ni_file::nis::{schema::Repository, ItemContainer};

#[test]
#[ignore]
fn test_nis_kontakt_custom_dir() -> Result<(), Box<dyn std::error::Error>> {
    for path in utils::get_test_files("../ni-file-data/NISound/**/Kontakt/**/*.*")? {
        dbg!(&path);

        let container = ItemContainer::read(std::fs::File::open(path)?)?;
        let _repository = Repository::from(container.clone());

        // assert_eq!(nis.authoring_application()?, AuthoringApplication::Kontakt);
    }

    Ok(())
}

#[test]
#[ignore]
fn test_nis_read_all() -> Result<(), Box<dyn std::error::Error>> {
    for path in utils::get_test_files("../ni-file-data/NISound/**/*.*")? {
        dbg!(&path);
        let container = ItemContainer::read(std::fs::File::open(path)?)?;
        let repository = Repository::from(container.clone());

        let root = repository.repository_root().unwrap()?;
        assert_eq!(root.repository_magic, 0);
        assert_eq!(root.repository_type, 1);

        match repository.infer_schema() {
            ni_file::nis::schema::NISObject::AppSpecific(_) => todo!(),
            ni_file::nis::schema::NISObject::Repository(_) => todo!(),
            ni_file::nis::schema::NISObject::BNISoundPreset(_) => todo!(),
            ni_file::nis::schema::NISObject::Preset(_) => todo!(),
            ni_file::nis::schema::NISObject::PresetChunkItem(_) => todo!(),
            ni_file::nis::schema::NISObject::Unknown => todo!(),
        }
    }

    Ok(())
}
