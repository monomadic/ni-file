use std::io::Cursor;

// use flate2::bufread::ZlibDecoder;

use crate::{
    kontakt::chunkdata::ChunkData, nks::meta_info::BPatchMetaInfoHeader, read_bytes::ReadBytesExt,
    Error, NIFileError,
};

use super::{error::NKSError, header::NKSHeader};

#[derive(Debug)]
pub struct NKSFile {
    pub header: NKSHeader,
    pub compressed_patch_data: Vec<u8>,
    pub meta_info: BPatchMetaInfoHeader,
}

impl NKSFile {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, NIFileError> {
        let magic = reader.read_u32_le()?;

        match magic {
            0x5EE56EB3 => {
                todo!();
            }
            0x7fa89012 => {}
            _ => {
                return Err(NKSError::InvalidMagicNumber(magic).into());
            }
        }

        let zlib_length = reader.read_u32_le()? as usize;
        let header = NKSHeader::read_le(&mut reader)?;

        match &header {
            NKSHeader::BPatchHeaderV2(_) => {
                todo!();
                // let mut compressed_data = Vec::new();
                // reader.read_to_end(&mut compressed_data)?;
                // // note: decompress with zlib-flate -uncompress < in > out
                // //          (from the qpdf package)
                // let mut decoder = ZlibDecoder::new(compressed_data.as_slice());
                //
                // // store decompressed data in byte slice
                // let data = decoder.read_sized_data()?;
                //
                // Ok(NKSFile {
                //     header,
                //     compressed_patch_data: data,
                //     meta_info: BPatchMetaInfoHeader::read(&mut reader)?,
                // })
            }

            NKSHeader::BPatchHeaderV42(h) => {
                // deflate InternalPatchData
                let data = crate::deflate::deflate_with_lib(
                    reader.read_bytes(zlib_length)?.as_slice(),
                    h.decompressed_length as usize,
                )?;

                Ok(NKSFile {
                    header,
                    compressed_patch_data: data,
                    meta_info: BPatchMetaInfoHeader::read(&mut reader)?,
                })
            }
        }
    }

    pub fn decompress_patch_chunks(&self) -> Result<Vec<ChunkData>, Error> {
        let mut objects = Vec::new();
        let mut compressed_data = Cursor::new(&self.compressed_patch_data);

        while let Ok(chunk) = ChunkData::read(&mut compressed_data) {
            objects.push(chunk);
        }

        Ok(objects)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_nksfile_read_v42() -> Result<(), NIFileError> {
        let file = File::open("tests/filetype/NKS/KontaktV42/4.2.4.5316-000.nki")?;
        let _nks = NKSFile::read(file)?;
        Ok(())
    }
}
