use std::collections::HashMap;

use crate::Error;

use super::{
    chunkdata::ChunkData,
    filename_list::{FNTableImpl, FileNameListPreK51},
    objects::bprogram::Program,
};

pub struct KontaktInstrument(pub Vec<ChunkData>);

impl KontaktInstrument {
    pub fn find_chunk(&self, id: u16) -> Option<&ChunkData> {
        self.0.iter().find(|c| c.id == id)
    }

    pub fn program(&self) -> Option<Result<Program, Error>> {
        self.find_chunk(0x4b).map(Program::try_from)
    }

    pub fn filename_table(&self) -> Option<Result<HashMap<u32, String>, Error>> {
        if let Some(chunk) = self.find_chunk(0x4b) {
            return Some(FNTableImpl::try_from(chunk).map(|f| f.filenames));
        }

        if let Some(chunk) = self.find_chunk(0x3d) {
            return Some(FileNameListPreK51::try_from(chunk).map(|f| f.filenames));
        }

        None
    }

    // pub fn zones(&self) -> Result<Vec<Zone>, Error> {
    //     todo!()
    // }
}

pub struct Zone {}
