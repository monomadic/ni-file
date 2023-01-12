use ni_file::{ni_object::*, prelude::*};

#[test]
fn test_reading_files() -> Result<()> {
    for entry in glob::glob("data/files/**/*.nki")? {
        let path = entry?;
        println!("test reading {:?}", path.display());

        let object: Result<NIObject> = std::fs::read(path)?.try_into();
        assert!(object.is_ok())
    }

    Ok(())
}
