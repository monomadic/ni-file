#[cfg(test)]
mod tests {
    use crate::read_bytes::ReadBytesExt;
    use std::{error::Error, fs::write};

    #[test]
    fn test_compress_with_lib() {
        let compressed_input =
            include_bytes!("../tests/data/compressed/kontakt-4/001-garbo2.compressed");
        let expected_output =
            include_bytes!("../tests/data/compressed/kontakt-4/001-garbo2.compressed");
        let mut buffer: [u8; 10000] = [0; 10000];

        fastlz::decompress(compressed_input, &mut buffer).expect("compression failed");

        let mut decompressed_output = buffer.as_slice();
        // trim zero padding outsize sized field
        let len = decompressed_output.scan_u32_le().unwrap() as usize;
        let output = decompressed_output.read_bytes(len).unwrap();

        assert_eq!(expected_output.to_vec(), output);
    }

    #[test]
    fn test_decompress_with_lib() -> Result<(), Box<dyn Error>> {
        let decompressed =
            include_bytes!("../tests/data/decompressed/kontakt-4/001-garbo2.decompressed");
        // let mut compressed = Vec::with_capacity(decompressed.len() * 2);

        let output_buffer_len = decompressed.len() * 2;
        let mut compressed_buffer: [u8; 4000] = [0; 4000];

        fastlz::compress(decompressed, &mut compressed_buffer).unwrap();

        write("output", compressed_buffer);
        //        panic!("{:?}", compressed_buffer);

        panic!("{:?}", compressed_buffer);

        // assert_eq!(input, output.as_slice());
        Ok(())
    }
}
