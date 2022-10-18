use ni_file::ni_segment::SegmentType;

#[test]
fn test_kontakt_segment() {
    let file = include_bytes!("./data/ni_container/kontakt-5.4-demo.nki");
    let container = ni_file::ni_container::NIContainer::read(file).expect("file to read");
    assert_eq!(
        container.data.data_chunk.type_id,
        SegmentType::RepositoryRoot
    );
}
