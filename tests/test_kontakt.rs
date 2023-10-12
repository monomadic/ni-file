use std::fs::File;

use ni_file::kontakt::{chunk_set::KontaktChunks, KontaktObject};

mod utils;

#[test]
#[ignore]
fn test_kontakt_obj_version_is_schema() -> Result<(), Box<dyn std::error::Error>> {
    for path in utils::get_test_files("../ni-file-data/Kontakt/NKI/*.*")? {
        dbg!(&path);

        let file = File::open(&path)?;
        let kontakt = KontaktChunks::read(file)?;

        for chunk in &kontakt.0 {
            match chunk.into_type()? {
                KontaktObject::Program(program) => match program.version() {
                    0x80 => assert_eq!(program.children().len(), 5),
                    0xA5 => assert_eq!(program.children().len(), 27),
                    0xAB => assert_eq!(program.children().len(), 27),
                    0xAF => assert_eq!(program.children().len(), 28),
                    _ => {
                        panic!("{:x}", program.version());
                    }
                },
                _ => (),
            };
        }
    }

    Ok(())
}
