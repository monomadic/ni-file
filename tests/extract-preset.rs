use ni_file::container::Container;

#[test]
fn test_kontakt_5() {
    let file = include_bytes!("./data/ni_container/kontakt-5.4-demo.nki");
    let container = ni_file::ni_repository::Repository::read(file);

    let container = container.expect("file to read");

    let container: Container = container.into();
    panic!("{:?}", container);

    // assert_eq!(format!("{:?}", container), "".to_string());
}
