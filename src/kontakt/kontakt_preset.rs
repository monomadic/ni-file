use std::{collections::HashMap, io::Cursor};

use crate::Error;

use super::{
    chunkdata::ChunkData,
    filename_list::{FNTableImpl, FileNameListPreK51},
};

pub struct KontaktInstrument(pub Vec<ChunkData>);

impl KontaktInstrument {
    pub fn filename_table(&self) -> Result<Option<HashMap<u32, String>>, Error> {
        if let Some(fnchunk) = self.0.iter().find(|c| c.id == 0x4b) {
            let reader = Cursor::new(&fnchunk.data);
            let fntable = FNTableImpl::read(reader)?;
            return Ok(Some(fntable.filenames));
        }

        if let Some(fnchunk) = self.0.iter().find(|c| c.id == 0x3d) {
            let reader = Cursor::new(&fnchunk.data);
            let fntable = FileNameListPreK51::read(reader)?;
            return Ok(Some(fntable.filenames));
        }
        Ok(None)
    }

    // pub fn zones(&self) -> Result<Vec<Zone>, Error> {
    //     todo!()
    // }
}

pub struct Zone {}
