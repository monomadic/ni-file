mod utils;

use ni_file::{kontakt::KontaktPreset, nks::container::NKSContainer};
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
            KontaktPreset::KontaktV2(_) => todo!(),
            KontaktPreset::Kon3(_) => todo!(),
            KontaktPreset::Kon4(_) => todo!(),
            KontaktPreset::Kon5(_) => todo!(),
            KontaktPreset::Kon6(_) => todo!(),
            KontaktPreset::Kon7(_) => todo!(),
            KontaktPreset::KontaktV42(_) => todo!(),
        }

        // match NKSContainer::read(file)?.header {
        //     BPatchHeader::BPatchHeaderV1(_) => todo!(),
        //     BPatchHeader::BPatchHeaderV2(_) => todo!(),
        //     BPatchHeader::BPatchHeaderV42(_) => todo!(),
        // };
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
            KontaktPreset::KontaktV2(_) => todo!(),
            KontaktPreset::Kon3(_) => todo!(),
            KontaktPreset::Kon4(_) => todo!(),
            KontaktPreset::Kon5(_) => todo!(),
            KontaktPreset::Kon6(_) => todo!(),
            KontaktPreset::Kon7(_) => todo!(),
            KontaktPreset::KontaktV42(_) => todo!(),
        }
    }

    Ok(())
}
