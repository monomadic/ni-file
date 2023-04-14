pub fn decompress(
    compressed_input: &[u8],
    decompressed_len: usize,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = vec![0; decompressed_len];
    // TODO: remove expect, return error
    fastlz::decompress(compressed_input, &mut buffer).expect("decompression failed");
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::read_bytes::ReadBytesExt;

    #[test]
    fn test_decompress_with_lib() {
        crate::utils::setup_logger();

        let compressed_input =
            include_bytes!("../tests/data/nisound/fastlz/kontakt-4/001-garbo2.compressed");
        let expected_output =
            include_bytes!("../tests/data/nisound/fastlz/kontakt-4/001-garbo2.decompressed");
        let decompressed_output = decompress(compressed_input, expected_output.len()).unwrap();

        assert_eq!(expected_output.to_vec(), decompressed_output);
    }

    // #[test]
    // fn test_compress_with_lib() {
    //     crate::utils::setup_logger();
    //
    //     let decompressed_input =
    //         include_bytes!("../tests/data/fastlz/kontakt-4/001-garbo2.decompressed");
    //     let expected_output =
    //         include_bytes!("../tests/data/fastlz/kontakt-4/001-garbo2.compressed");
    //
    //     // let output_buffer_len = decompressed_input.len() * 2;
    //     let mut compressed_buffer: [u8; 4000] = [0; 4000];
    //
    //     fastlz::compress(decompressed_input, &mut compressed_buffer).unwrap();
    //
    //     let mut buf: [u8; 4] = [0; 4];
    //     buf.copy_from_slice(&compressed_buffer[0..4]);
    //     let len = u32::from_le_bytes(buf);
    //
    //     let mut compressed_output = compressed_buffer.as_slice();
    //     log::debug!("read len field: {:?}", len);
    //     let output = compressed_output.read_bytes(len as usize).unwrap();
    //
    //     assert_eq!(expected_output.to_vec(), output);
    // }
}
