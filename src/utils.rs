use crate::prelude::*;
use std::path::PathBuf;

#[allow(dead_code)]
pub(crate) fn get_test_files() -> Result<Vec<PathBuf>> {
    let path = "tests/data/files/**/*.*";
    Ok(glob::glob(path)
        .map_err(|_| NIFileError::Generic(format!("error globbing: {}", path)))?
        .filter_map(|path| path.ok())
        .filter(|path| path.file_name().unwrap() != ".DS_Store")
        .collect())
}

#[allow(dead_code)]
pub(crate) fn get_files(path: &str) -> Result<Vec<PathBuf>> {
    let files: Vec<PathBuf> = glob::glob(path)
        .map_err(|_| NIFileError::Generic(format!("error globbing: {}", path)))?
        .filter_map(|path| path.ok())
        .filter(|path| path.is_file())
        .filter(|path| path.file_name().unwrap() != ".DS_Store")
        .collect();

    if files.is_empty() {
        return Err(NIFileError::Generic(format!("no files found at: {}", path)));
    }

    Ok(files)
}

#[allow(dead_code)]
pub fn format_hex(buffer: &[u8]) -> String {
    format!(
        "{}",
        &buffer
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect::<String>()
    )
}

#[allow(dead_code)]
fn format_hex_spaced(buffer: &[u8]) -> String {
    buffer
        .iter()
        .enumerate()
        .map(|(index, &byte)| {
            let hex_byte = format!("{:02x}", byte);
            // Insert an additional space after every 4th byte (8 hex characters)
            if index > 0 && (index + 1) % 4 == 0 {
                format!("{} ", hex_byte)
            } else {
                hex_byte
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

#[allow(dead_code)]
fn hex_string_to_bytes(hex: &str) -> Result<Vec<u8>> {
    // Remove "0x" prefix if it exists.
    let hex = if hex.starts_with("0x") || hex.starts_with("0X") {
        &hex[2..]
    } else {
        hex
    };

    // Ensure that the length is even, because hex characters represent
    // nibbles (half-bytes) and there must be an even number of them.
    if hex.len() % 2 == 1 {
        return Err(NIFileError::Static("Invalid Hex Digit"));
    }

    // Convert every two characters into a byte.
    let mut bytes = Vec::new();
    for i in (0..hex.len()).step_by(2) {
        let byte_str = &hex[i..i + 2];
        let byte =
            u8::from_str_radix(byte_str, 16).map_err(|_| NIFileError::Static("Hex Error"))?;
        bytes.push(byte);
    }

    Ok(bytes)
}
