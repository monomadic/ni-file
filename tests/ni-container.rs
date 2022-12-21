use ni_file::ni_object::*;
use ni_file::prelude::*;

#[test]
fn ni_object_test() -> Result<()> {
    let file = include_bytes!("./data/ni_container/kontakt-5.4-demo.nki").as_slice();
    let _object: NIObject = file.try_into()?;

    // panic!("{:?}", object);

    Ok(())
}
