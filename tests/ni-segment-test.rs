// 01 00 00 00 01 08 00 00 00 01 00 00 00 0a 00 00 00 31 00 2e 00 32 00 2e 00 30 00 2e 00 31 00 30 00 31 00 30 00

fn test_segment_xxx() {
    let file = include_bytes!("./data/ni_container/kontakt-4--booga.nki");
    let container = ni_file::ni_container::read(file).unwrap();
}