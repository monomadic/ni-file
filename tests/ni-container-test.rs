#[test]
fn test_kontakt_4_booga() {
    let file = include_bytes!("./data/k4-booga.nki");
    let container = ni_file::ni_container::read(file).unwrap();

    assert_eq!(container.length, 2440);
    assert_eq!(container.unknown_a, 0);
    assert_eq!(container.unknown_b, 1);
    assert_eq!(&container.tag(), "hsin");
    assert_eq!(container.id, 1);
    assert_eq!(container.unknown_c, 0);

    assert_eq!(container.to_string(), "<hsin>");

    // assert_eq!("", format!("{:?}", container));
}