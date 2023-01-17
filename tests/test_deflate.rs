
#[test]
fn test_kontakt_4_booga() {
    let compressed_file = include_bytes!("data/compressed/k4-garbo2.compressed");
    let decompressed_file = include_bytes!("data/decompressed/k4-garbo2.decompressed");

    let (_rem, result) = ni_file::deflate::deflate(compressed_file, 11).unwrap();
    assert_eq!(&result, decompressed_file);
}
