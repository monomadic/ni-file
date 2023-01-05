use binread::io::Cursor;

use ni_file::prelude::*;
use ni_file::raw_data::NIData;

#[test]
fn test_kontakt5_repository_root() -> Result<()> {
    let file = include_bytes!("../data/item-frames/118-Kontakt5-RepositoryRoot.data");
    let mut cursor: Cursor<&[u8]> = Cursor::new(file.as_slice());

    let _data: NIData = ni_file::raw_data::read_data(&mut cursor)?;

    Ok(())
}
