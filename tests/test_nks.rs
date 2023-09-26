mod utils;

use ni_file::nks::container::NKSContainer;
use std::fs;

#[test]
fn test_reading_nks() -> Result<(), Box<dyn std::error::Error>> {
    for path in utils::get_test_files("tests/filetype/NKS/KontaktV2/*.*")? {
        dbg!(&path);
        let file = fs::File::open(path.as_path())?;
        match NKSContainer::read(file)?.header {
            ni_file::nks::header::BPatchHeader::BPatchHeaderV1(_) => todo!(),
            ni_file::nks::header::BPatchHeader::BPatchHeaderV2(_) => todo!(),
            ni_file::nks::header::BPatchHeader::BPatchHeaderV42(_) => todo!(),
        };
    }

    Ok(())
}
