use crate::NIFileError;

pub fn decompress(
    compressed_input: &[u8],
    decompressed_len: usize,
) -> Result<Vec<u8>, NIFileError> {
    let mut buffer = vec![0; decompressed_len];

    fastlz::decompress(compressed_input, &mut buffer)
        .map_err(|_| NIFileError::DecompressionError)?;

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress_with_lib() {
        let compressed_input =
            include_bytes!("../tests/data/nisound/fastlz/kontakt-4/001-garbo2.compressed");
        let expected_output =
            include_bytes!("../tests/data/nisound/fastlz/kontakt-4/001-garbo2.decompressed");
        let decompressed_output = decompress(compressed_input, expected_output.len()).unwrap();

        assert_eq!(expected_output.to_vec(), decompressed_output);
    }
}
