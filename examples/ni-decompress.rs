pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(path) = std::env::args().nth(1) else {
        println!("usage: ni-deflate <FILE> <SIZE>");
        return Ok(());
    };

    let Some(length) = std::env::args().nth(2) else {
        println!("usage: ni-deflate <FILE> <SIZE>");
        return Ok(());
    };

    let compressed_input = std::fs::read(&path)?;

    let output =
        ni_file::deflate::deflate_with_lib(compressed_input.as_slice(), length.parse::<usize>()?)?;

    // let output =
    //     ni_file::deflate::deflate_checked(compressed_input.as_slice(), length.parse::<usize>()?)
    //         .expect("decompression failed");

    std::fs::write("dump", output)?;
    println!("file decompresed and written successfully.");

    Ok(())
}
