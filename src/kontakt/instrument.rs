use std::collections::HashMap;

use crate::Error;

use super::{
    chunk::Chunk,
    objects::{
        filename_list::{FNTableImpl, FileNameListPreK51},
        program::Program,
    },
};

pub struct KontaktInstrument(pub Vec<Chunk>);

impl KontaktInstrument {
    pub fn find_first(&self, id: u16) -> Option<&Chunk> {
        self.0.iter().find(|c| c.id == id)
    }

    pub fn find(&self, id: u16) -> Vec<&Chunk> {
        self.0.iter().filter(|c| c.id == id).collect()
    }

    pub fn program(&self) -> Option<Result<Program, Error>> {
        self.find_first(0x28).map(Program::try_from)
    }

    pub fn filename_tables(&self) -> Result<Option<FNTableImpl>, Error> {
        if let Some(chunk) = self.find_first(0x4b) {
            return Ok(Some(FNTableImpl::try_from(chunk)?));
        }

        // TODO: convert FileNameListPreK51

        Ok(None)
    }

    pub fn filename_table(&self) -> Option<Result<HashMap<u32, String>, Error>> {
        if let Some(chunk) = self.find_first(0x4b) {
            return Some(FNTableImpl::try_from(chunk).map(|f| f.sample_filetable));
        }

        if let Some(chunk) = self.find_first(0x3d) {
            return Some(FileNameListPreK51::try_from(chunk).map(|f| f.filenames));
        }

        None
    }
}

pub struct Zone {}
