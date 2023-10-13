pub(crate) struct RIFFHeader {
    file_type: String,
    size: u32,
}

pub(crate) struct RIFFChunk {
    id: String,
    size: u32,
    data: Vec<u8>,
}

// fn parse_iff<R: Read + Seek>(mut reader: R) -> Result<Vec<RIFFChunk>, std::io::Error> {
//     let mut chunks = Vec::new();
//
//     // Read header
//     let mut header = [0; 4];
//     reader.read_exact(&mut header)?;
//     let file_type = std::str::from_utf8(&header)?.to_owned();
//
//     let mut size_bytes = [0; 4];
//     reader.read_exact(&mut size_bytes)?;
//     let size = u32::from_le_bytes(size_bytes);
//
//     let header = RIFFHeader { file_type, size };
//
//     // Parse chunks
//     loop {
//         let mut id = [0; 4];
//         reader.read_exact(&mut id)?;
//         let id = std::str::from_utf8(&id)?.to_owned();
//
//         let mut size_bytes = [0; 4];
//         reader.read_exact(&mut size_bytes)?;
//         let size = u32::from_le_bytes(size_bytes);
//
//         let mut data = vec![0; size as usize];
//         reader.read_exact(&mut data)?;
//
//         let chunk = RIFFChunk { id, size, data };
//         chunks.push(chunk);
//
//         if id == "END!" {
//             break;
//         }
//     }
//
//     Ok(chunks)
// }
