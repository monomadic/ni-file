use ni_file::prelude::*;

#[test]
fn read_field_headers_test() -> Result<()> {
    // include_bytes!("../data/item-data/118-106-Kontakt5-RepositoryRoot-Authorization.data");
    let file = include_bytes!("../data/item-frames/118-Kontakt5-RepositoryRoot.data");
    let fields = ni_file::raw_data::read(file)?;

    assert_eq!(fields.len(), 3);
    assert_eq!(fields[0].data, vec![1, 0, 0, 0]);
    assert_eq!(fields[0].data.len(), 4);

    assert_eq!(fields[1].data[0], 1);
    assert_eq!(fields[1].data.len(), 28);

    assert_eq!(fields[2].data[4], 7);
    assert_eq!(fields[2].data.len(), 58);

    Ok(())
}
