use crate::prelude::io;
use crate::{read_bytes::ReadBytesExt, Error, NIFileError};

use super::{pubdata::PubData, structured_object_data::StructuredObjectData};

#[doc = include_str!("../../doc/schematics/nks-objects/StructuredObject.md")]
#[derive(Debug)]
pub struct StructuredObject {
    pub id: u16,
    pub version: u16,
    pub public_data: Vec<u8>,
    pub private_data: Vec<u8>,
    pub children: Vec<StructuredObject>,
}

impl StructuredObject {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        // let current_position = reader.seek(io::SeekFrom::End(0))?;
        let id = reader.read_u16_le()?;
        let length = reader.read_u32_le()? as usize;

        // Check that file has at least `length` bytes

        let is_data_structured = reader.read_bool()?;

        if !is_data_structured {
            return Ok(Self {
                id,
                public_data: reader.read_bytes(length - 1)?,
                version: 0,
                private_data: Vec::new(),
                children: Vec::new(),
            });
        }

        let public_data_version = reader.read_u16_le()?;
        println!("  version: 0x{public_data_version:x}");

        let private_data_length = reader.read_u32_le()?;
        let private_data = reader
            .read_bytes(private_data_length as usize)
            .map_err(|e| {
                NIFileError::Generic(format!(
                    "Failed to read StructuredObject private_data: length={private_data_length} error={e}",
                ))
            })?;

        let public_data_length = reader.read_u32_le()?;
        let public_data = reader
            .read_bytes(public_data_length as usize)
            .map_err(|e| {
                NIFileError::Generic(format!(
                    "Failed to read StructuredObject public_data: length={public_data_length} version={public_data_version} error={e}",
                ))
            })?;

        let children_data_length = reader.read_u32_le()?;
        let children_data = reader
            .read_bytes(children_data_length as usize)
            .map_err(|e| {
                NIFileError::Generic(format!(
                    "Failed to read StructuredObject private_data: length={children_data_length} error={e}",
                ))
            })?;
        let mut children_reader = io::Cursor::new(children_data);

        let mut children = Vec::new();
        while let Ok(object) = StructuredObject::read(&mut children_reader) {
            children.push(object);
        }

        Ok(Self {
            id,
            private_data,
            version: public_data_version,
            public_data,
            children,
        })
    }

    pub fn pubdata(&self) -> Result<Option<PubData>, Error> {
        Ok(Some(PubData::from(
            io::Cursor::new(&self.public_data),
            self.id,
            self.version,
        )?))
    }

    pub fn data(self) -> Result<StructuredObjectData, Error> {
        Ok(StructuredObjectData::try_from(self)?)
    }

    // pub fn factory(id: u32, length: u32) -> Option<Self> {
    //     match id {
    //         0..=0xe => match id {
    //             5 => {
    //                 // Add logic for id=5
    //                 todo!();
    //             }
    //             6 => {
    //                 todo!();
    //                 // return Some(StructuredObjectType::PublicObject(PublicObject {
    //                 //     id,
    //                 //     length,
    //                 //     // other initializations...
    //                 // }));
    //             }
    //             _ => {
    //                 // Add logic for other cases...
    //                 todo!();
    //             }
    //         },
    //         0xf..=0x2a => {
    //             if id == 0xf {
    //                 // Add logic for id=0xf
    //                 todo!();
    //             }
    //         }
    //         // Add other cases...
    //         0x2b | 0x3f | 0x41 => {
    //             todo!();
    //             // return Some(StructuredObjectType::PublicObject(PublicObject {
    //             //     id,
    //             //     length,
    //             //     // other initializations...
    //             // }));
    //         }
    //         // Add more cases...
    //         _ => {}
    //     }
    //
    //     None
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::format_hex;

    #[test]
    fn test_structured_object_0x28() -> Result<(), Error> {
        let mut file = io::Cursor::new(include_bytes!(
            "../../tests/patchdata/KontaktV42/StructuredObject/0x28"
        ));
        let obj = StructuredObject::read(&mut file)?;

        assert_eq!(obj.id, 0x28);
        assert_eq!(obj.version, 0x80);
        assert_eq!(obj.children.len(), 3);

        println!("public_data: {}", format_hex(&obj.public_data));
        println!("private_data: {}", format_hex(&obj.private_data));

        for child in obj.children {
            println!("{child:?}");
            child.pubdata()?;

            // for child in child.children {
            //     crate::kontakt::pubdata::PubData::from(child.data.as_slice(), child.id, child.version)?;
            // }
        }

        // TODO: test file is read to end

        Ok(())
    }

    #[test]
    fn test_structured_object_0x3d() -> Result<(), Error> {
        let mut file = io::Cursor::new(include_bytes!(
            "../../tests/patchdata/KontaktV42/StructuredObject/0x3D"
        ));
        let obj = StructuredObject::read(&mut file)?;

        assert_eq!(obj.id, 0x3d);
        assert_eq!(obj.version, 0x00);
        assert_eq!(obj.children.len(), 0);

        assert!(obj.pubdata().is_ok());

        // TODO: test file is read to end

        Ok(())
    }

    #[test]
    fn test_structured_object_0x25() -> Result<(), Error> {
        let mut file = io::Cursor::new(include_bytes!(
            "../../tests/patchdata/KontaktV42/StructuredObject/0x25"
        ));
        let obj = StructuredObject::read(&mut file)?;

        assert_eq!(obj.id, 0x25);
        assert_eq!(obj.version, 0x50);
        assert_eq!(obj.children.len(), 1);

        println!("public_data: {}", format_hex(&obj.public_data));
        println!("private_data: {}", format_hex(&obj.private_data));

        // TODO: test file is read to end

        Ok(())
    }
}

// // pub struct StructuredObjectReader {
// //     pub id: u16,
// //     pub length: u32,
// // }
// //
// // impl StructuredObjectReader {
// //     /// Emulates StructuredObject::doRead(StructuredObject *this, Stream *stream)
// //     pub fn do_read<R: ReadBytesExt>(&self, mut reader: R) -> Result<(), Error> {
// //         println!("\nStructuredObject::doRead() {}", self.id);
// //
// //         let is_chunked = reader.read_bool()?;
// //         println!("is_chunked {:?}", is_chunked);
// //
// //         if is_chunked {
// //             let object_version = reader.read_u16_le()?;
// //             let object_length = reader.read_u32_le()?;
// //             if object_length > 0 {
// //                 let _private_data = reader.read_bytes(object_length as usize)?;
// //             }
// //
// //             let public_data_length = reader.read_u32_le()?;
// //             if public_data_length > 0 {
// //                 println!(
// //                     "{:?}",
// //                     PubData::create(&mut reader, self.id, object_version)?
// //                 );
// //             }
// //         } else {
// //             let length = self.length - 1; // to account for the boolean
// //             let _data = reader.read_bytes(length as usize)?;
// //         }
// //
// //         Ok(())
// //     }
// // }
//
// // pub enum StructuredObjectType {
// //     Unknown,
// // }
// //
// // impl StructuredObjectType {
// //     pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<(), Error> {
// //         let id = reader.read_u16_le()?;
// //         println!("Reading StructuredObject id:0x{:x}", &id);
// //
// //         let length = reader.read_u32_le()?;
// //         println!("length {}", length);
// //
// //         match id {
// //             0x28 => {
// //                 // read the chunk into memory
// //                 let _reader = reader.read_bytes(length as usize)?;
// //             }
// //             0x3d => {
// //                 FileNameListPreK51::read(&mut reader)?;
// //             }
// //             _ => panic!("Unsupported StructuredObject: 0x{:x}", id),
// //         }
// //
// //         Ok(())
// //     }
// // }
//
// impl StructuredObject {
//     // pub fn do_read_unchecked<R: ReadBytesExt>(
//     //     mut reader: R,
//     //     id: u16,
//     //     length: u32,
//     // ) -> Result<(), Error> {
//     //     println!(
//     //         "StructuredObject::do_read_unchecked 0x{:x} {} bytes",
//     //         id, length
//     //     );
//     //
//     //     // let mut chunk = ChunkData::read(&mut reader)?;
//     //     // let reader = chunk.data;
//     //
//     //     let item_version = reader.read_u16_le()?;
//     //
//     //     // PRIVATE DATA
//     //     let length = reader.read_u32_le()?;
//     //     println!("private data length {:?}", length);
//     //     let _private_data = reader.read_bytes(length as usize)?;
//     //
//     //     // PUBLIC DATA
//     //     println!("public data length {:?}", reader.read_u32_le()?);
//     //     println!("{:?}", PubData::create(&mut reader, id, item_version)?);
//     //
//     //     // read all children into memory
//     //     let children_data_length = reader.read_u32_le()? as usize;
//     //     let children_data = reader.read_bytes(children_data_length)?;
//     //     let mut reader = children_data.as_slice();
//     //
//     //     // StructuredObject::factory
//     //     while let Ok(chunk_id) = reader.read_u16_le() {
//     //         let chunk_length = reader.read_u32_le()?;
//     //         let chunk_data = reader.read_bytes(chunk_length as usize)?;
//     //
//     //         println!("chunk data {} {}", chunk_id, chunk_length);
//     //
//     //         println!(
//     //             "{:?}",
//     //             StructuredObject::do_read(&mut chunk_data.as_slice(), chunk_id, chunk_length)?
//     //         );
//     //     }
//     //
//     //     Ok(())
//     // }
//     //
//     // pub fn do_read<R: ReadBytesExt>(mut reader: R, id: u16, length: u32) -> Result<(), Error> {
//     //     println!("StructuredObject::do_read 0x{:x} {} bytes", id, length);
//     //
//     //     let reader = reader.read_bytes(length as usize)?;
//     //     let mut reader = reader.as_slice();
//     //
//     //     if reader.read_bool()? {
//     //         Self::do_read_unchecked(&mut reader, id, length - 1)?;
//     //
//     //         // match id {
//     //         //     0x28 => {
//     //         //         let item_version = reader.read_u16_le()?;
//     //         //
//     //         //         // PRIVATE DATA
//     //         //         let length = reader.read_u32_le()?;
//     //         //         println!("private data length {:?}", length);
//     //         //         let _private_data = reader.read_bytes(length as usize)?;
//     //         //
//     //         //         // PUBLIC DATA
//     //         //         println!("public data length {:?}", reader.read_u32_le()?);
//     //         //         println!("{:?}", PubData::create(&mut reader, 0x28, item_version)?);
//     //         //
//     //         //         // read all children into memory
//     //         //         let children_data_length = reader.read_u32_le()? as usize;
//     //         //         let children_data = reader.read_bytes(children_data_length)?;
//     //         //         let mut reader = children_data.as_slice();
//     //         //
//     //         //         // StructuredObject::factory
//     //         //         while let Ok(chunk_id) = reader.read_u16_le() {
//     //         //             let item_length = reader.read_u32_le()?;
//     //         //             let item_data = reader.read_bytes(item_length as usize)?;
//     //         //
//     //         //             match chunk_id {
//     //         //                 0x32 => {
//     //         //                     println!("{:?}", VoiceGroups::read(&mut item_data.as_slice())?);
//     //         //                 }
//     //         //                 0x33 => println!("0x{:x} GroupList", chunk_id),
//     //         //
//     //         //                 0x34 => {
//     //         //                     println!("{:?}", ZoneList::read(&mut item_data.as_slice())?);
//     //         //                 }
//     //         //
//     //         //                 0x35 | 0x48 | 0x49 | 0x4e => {
//     //         //                     println!("0x{:x} PrivateRawObject", chunk_id)
//     //         //                 }
//     //         //
//     //         //                 0x36 => panic!("0x36 ProgramList"),
//     //         //                 0x37 => println!("0x37 SlotList"),
//     //         //                 0x38 => println!("0x38 StartCritList"),
//     //         //                 0x39 => println!("0x39 LoopArray"),
//     //         //
//     //         //                 0x3a => {
//     //         //                     // 0x3a => println!("0x3a BParamArray<8>"),
//     //         //                     BParamArray::read(&mut item_data.as_slice(), 8)?;
//     //         //                 }
//     //         //
//     //         //                 0x3b => println!("0x3b BParamArray<16>"),
//     //         //                 0x3c => println!("0x3c BParamArray<32>"),
//     //         //                 0x3d => println!("0x3d FileNameListPreK51"),
//     //         //
//     //         //                 0x41 => println!("0x41 PublicObject"),
//     //         //
//     //         //                 0x45 => {
//     //         //                     Self::do_read(&mut reader, chunk_id, 0x58)?;
//     //         //                 }
//     //         //
//     //         //                 0x4b => println!("0x4b FileNameList"),
//     //         //
//     //         //                 _ => panic!("Unsupported StructuredObject::factory(0x{:x})", chunk_id),
//     //         //             }
//     //         //         }
//     //         //     }
//     //         //     0x3d => {
//     //         //         // FileNameListPreK51
//     //         //         println!("{:?}", FileNameListPreK51::read(&mut reader)?);
//     //         //     }
//     //         //     0x45 => {}
//     //         //     _ => panic!("Unknown StructuredObject 0x{:x}", id),
//     //         // }
//     //     } else {
//     //         println!("not read chunked");
//     //     }
//     //
//     //     Ok(())
//     // }
//     //
//     // pub fn factory(mut reader: R) -> Result<Self, Error> {
//     // }
//
//     // emulates StucturedObject::doRead
//     pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
//         println!("StructuredObject::read");
//
//         let ChunkData { id, data } = ChunkData::read(&mut reader)
//             .map_err(|_| NIFileError::Static("Failed to read StructuredObject data"))?;
//
//         let mut reader = data.as_slice();
//
//         let read_raw = reader.read_bool()?;
//         // assert!(!read_raw, "Reading raw object data unsupported!");
//
//         let ChunkData {
//             id: private_data_id,
//             data: private_data,
//         } = ChunkData::read(&mut reader).map_err(|e| {
//             NIFileError::Generic(format!(
//                 "Failed to read StructuredObject private data: {}",
//                 e
//             ))
//         })?;
//
//         let public_data_length = reader.read_u32_le()?;
//         let public_data = reader.read_bytes(public_data_length as usize)?;
//
//         // TODO: read children as objects
//         let children = Vec::new();
//
//         Ok(Self {
//             id,
//
//             private_data_id,
//             private_data,
//
//             public_data,
//             children,
//         })
//     }
// }
//
// // #[cfg(test)]
// // mod tests {
// //     use super::*;
// //
// //     #[test]
// //     fn test_kontakt_preset_read() -> Result<(), Error> {
// //         for path in crate::utils::get_files("tests/data/nisound/preset/kontakt/**/*")? {
// //             println!("\nreading {:?}", path);
// //
// //             let file = std::fs::File::open(&path)?;
// //
// //             let _chunks = StructuredObject::read(&file)?;
// //             let _chunks = StructuredObject::read(&file)?;
// //         }
// //
// //         Ok(())
// //     }
// // }
//
// // fn read_array<R: ReadBytesExt>(
// //     mut reader: R,
// //     items: usize,
// // ) -> Result<Vec<Option<(u16, Vec<u8>)>>, Error> {
// //     let mut array = Vec::with_capacity(items);
// //
// //     for _ in 0..items {
// //         array.push(match reader.read_bool()? {
// //             true => {
// //                 let item_id = reader.read_u16_le()?;
// //                 let item_length = reader.read_u32_le()?;
// //                 let item_data = reader.read_bytes(item_length as usize)?;
// //
// //                 Some((item_id, item_data))
// //             }
// //             false => None,
// //         })
// //     }
// //
// //     println!("array8: {} items {:?}", array.len(), array);
// //
// //     Ok(array)
// // }
//
// // fn read_chunk<R: ReadBytesExt>(mut reader: R) -> Result<Vec<u8>, Error> {
// //     let length = reader.read_u32_le()?;
// //     if length > 0 {
// //         Ok(reader.read_bytes(length as usize)?)
// //     } else {
// //         Ok(Vec::new())
// //     }
// // }
//
// #[test]
// fn test_structured_object() -> Result<(), Error> {
//     let mut file = include_bytes!("tests/StructuredObject/0x28").as_slice();
//     let obj = StructuredObject::read(&mut file)?;
//
//     assert_eq!(obj.id, 0x28);
//     assert_eq!(obj.private_data_id, 0x80);
//     assert_eq!(obj.children.len(), 0);
//
//     // TODO: test file is read to end
//
//     Ok(())
// }
