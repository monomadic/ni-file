use std::collections::HashMap;

use crate::Error;

use super::{
    chunkdata::ChunkData,
    filename_list::{FNTableImpl, FileNameListPreK51},
    objects::program::Program,
};

pub struct KontaktInstrument(pub Vec<ChunkData>);

impl KontaktInstrument {
    pub fn find_first(&self, id: u16) -> Option<&ChunkData> {
        self.0.iter().find(|c| c.id == id)
    }

    pub fn find(&self, id: u16) -> Vec<&ChunkData> {
        self.0.iter().filter(|c| c.id == id).collect()
    }

    pub fn program(&self) -> Option<Result<Program, Error>> {
        self.find_first(0x28).map(Program::try_from)
    }

    pub fn filename_table(&self) -> Option<Result<HashMap<u32, String>, Error>> {
        if let Some(chunk) = self.find_first(0x4b) {
            return Some(FNTableImpl::try_from(chunk).map(|f| f.filenames));
        }

        if let Some(chunk) = self.find_first(0x3d) {
            return Some(FileNameListPreK51::try_from(chunk).map(|f| f.filenames));
        }

        None
    }
}

pub struct Zone {}