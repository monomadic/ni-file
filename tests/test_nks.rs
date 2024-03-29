mod utils;

use ni_file::{kontakt::schemas::KontaktPreset, nks::container::NKSContainer};
use std::fs;

#[test]
fn test_reading_nks() -> Result<(), Box<dyn std::error::Error>> {
    for path in utils::get_test_files("tests/data/Containers/NKS/files/**/*.*")? {
        dbg!(&path);
        let file = fs::File::open(path.as_path())?;

        match NKSContainer::read(file)?.preset()? {
            KontaktPreset::KontaktV1(kon1) => {
                assert!(kon1.preset.to_string().len() > 0);
            }
            _ => todo!(),
        }
    }

    Ok(())
}

#[test]
#[ignore]
fn test_reading_nks_custom_dir() -> Result<(), Box<dyn std::error::Error>> {
    for path in utils::get_test_files("../test-data/kontakt/nks/**/*.*")? {
        dbg!(&path);
        let file = fs::File::open(path.as_path())?;
        let nks = NKSContainer::read(file)?;

        match nks.preset()? {
            KontaktPreset::KontaktV1(kon1) => {
                assert!(kon1.preset.to_string().len() > 0);
            }
            _ => todo!(),
        }
    }

    Ok(())
}
