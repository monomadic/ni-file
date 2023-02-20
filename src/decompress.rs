fn trim(buffer: &[u8]) -> Vec<u8> {
    vec![]
}

#[cfg(test)]
mod tests {
    use crate::read_bytes::ReadBytesExt;
    use std::{error::Error, fs::write};

    #[test]
    fn test_decompress_with_lib() {
        crate::utils::setup_logger();

        let compressed_input =
            include_bytes!("../tests/data/compressed/kontakt-4/001-garbo2.compressed");
        let expected_output =
            include_bytes!("../tests/data/decompressed/kontakt-4/001-garbo2.decompressed");
        let mut buffer: [u8; 10000] = [0; 10000];

        fastlz::decompress(compressed_input, &mut buffer).expect("compression failed");

        let mut decompressed_output = buffer.as_slice();
        // log::debug!("{:?}", decompressed_output);

        // trim zero padding outsize sized field
        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&buffer[0..4]);
        let len = u32::from_le_bytes(buf);
        log::debug!("read len field: {:?}", len);
        let output = decompressed_output.read_bytes(len as usize).unwrap();

        assert_eq!(expected_output.to_vec(), output);
    }

    #[test]
    fn test_compress_with_lib() {
        let decompressed_input =
            include_bytes!("../tests/data/decompressed/kontakt-4/001-garbo2.decompressed");
        let expected_output =
            include_bytes!("../tests/data/compressed/kontakt-4/001-garbo2.compressed");

        // let output_buffer_len = decompressed_input.len() * 2;
        let mut compressed_buffer: [u8; 4000] = [0; 4000];

        fastlz::compress(decompressed_input, &mut compressed_buffer).unwrap();

        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&compressed_buffer[0..4]);
        let len = u32::from_le_bytes(buf);

        let mut compressed_output = compressed_buffer.as_slice();
        log::debug!("read len field: {:?}", len);
        let output = compressed_output.read_bytes(len as usize).unwrap();

        assert_eq!(expected_output.to_vec(), output);
    }
}
