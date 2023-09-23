mod utils;

use ni_file::nks::nksfile::NKSContainer;
use std::fs;

#[test]
fn test_reading_nks() -> Result<(), Box<dyn std::error::Error>> {
    for path in utils::get_test_files("tests/filetype/NKS/KontaktV2/*.*")? {
        dbg!(&path);
        let file = fs::File::open(path.as_path())?;
        let nks = NKSContainer::read(file)?;
        match nks {
            NKSContainer::V1(v1) => {
                dbg!(v1.preset_xml().unwrap());
            }
            NKSContainer::V2(v2) => {
                dbg!(v2.header);
            }
            NKSContainer::V42(v42) => {
                dbg!(v42.header);
            }
        };
    }

    Ok(())
}
