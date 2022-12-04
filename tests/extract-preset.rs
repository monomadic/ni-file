#[test]
fn test_kontakt_5() {
    let file = include_bytes!("./data/ni_container/kontakt-5.4-demo.nki");
    let container = ni_file::ni_repository::Repository::read(file).expect("file to read");
    assert!(true);
}
