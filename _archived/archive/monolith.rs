use binread::{io::Cursor, BinReaderExt, NullString, NullWideString};
use std::io::BufRead; // consume

use std::io::Read;

pub(crate) fn read(buffer: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    info!("reading monolith table data {} bytes", buffer.len());

    let mut file_cursor = Cursor::new(buffer);

    // NI FC MTD
    {
        let mut header_section = [0; 256];
        file_cursor.read(&mut header_section)?;

        let mut header_cursor = binread::io::Cursor::new(header_section);
        let header_tag = header_cursor.read_be::<NullString>().unwrap().into_string();
        info!("monolith header: {:?}", header_tag);
    }

    let file_count: u64;
    let files_total_bytes: u64;

    // NI FC TOC
    {
        let mut section = [0; 640];
        file_cursor.read(&mut section)?;
        let mut cursor = binread::io::Cursor::new(section);

        let _unknown = cursor.read_le::<u64>().unwrap();
        let _unknown = cursor.read_le::<u64>().unwrap();
        file_count = cursor.read_le::<u64>().unwrap();
        files_total_bytes = cursor.read_le::<u64>().unwrap();

        info!("file count: {}", file_count);
        info!("{} files in monolith", file_count);
        info!("all files total size {}", files_total_bytes);

        let tag = cursor.read_be::<NullString>().unwrap().into_string();
        info!("tag: {:?}", tag);
    }

    let mut offsets = Vec::new();

    // get filenames and offsets
    for _i in 0..file_count {
        let mut section = [0; 640];
        file_cursor.read(&mut section)?;
        let mut cursor = binread::io::Cursor::new(section);

        let file_offset = cursor.read_le::<u64>().unwrap();
        let file_index = cursor.read_le::<u64>().unwrap();
        cursor.consume(16);
        let filename = cursor.read_le::<NullWideString>().unwrap().into_string();
        info!(
            "filename {} file_offset {} file_index {}",
            &filename, file_offset, file_index
        );
        offsets.push((file_offset, filename));
    }

    // NI FC TOC
    {
        let mut section = [0; 640];
        file_cursor.read(&mut section)?;
        let mut cursor = binread::io::Cursor::new(section);

        let file_start = cursor.read_le::<u64>().unwrap();
        let _unknown = cursor.read_le::<u64>().unwrap();
        let _file_start = cursor.read_le::<u64>().unwrap();
        let _files_total_bytes = cursor.read_le::<u64>().unwrap();

        info!("{} file_start", file_start);

        let tag = cursor.read_be::<NullString>().unwrap().into_string();
        info!("tag: {:?}", tag);
    }

    let mut iter = offsets.iter().peekable();

    while let Some((offset, filename)) = iter.next() {
        if let Some(next) = iter.peek() {
            let size = next.0 - offset;
            let mut filebuf: &mut Vec<u8> = &mut vec![0u8; size as usize];
            file_cursor.read(&mut filebuf)?;

            info!("wrote {} ({} bytes)", filename, filebuf.len());
            std::fs::write(filename, filebuf)?;
        } else {
            let filesize = buffer.len();
            info!("{} {}", filesize, file_cursor.position());
            let bytes_remaining = filesize - file_cursor.position() as usize;

            let mut buffer: &mut Vec<u8> = &mut vec![0u8; bytes_remaining as usize];
            file_cursor.read_exact(&mut buffer)?;
            info!("wrote {} ({} bytes)", filename, &buffer.len());
            std::fs::write(format!("output/{}", filename), buffer)?;
        }
    }
    Ok(())
}
