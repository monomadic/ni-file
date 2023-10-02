// Kon4 Schema:
//
// 0x28 Program
//  0x3A BParameterArray<BParFX,8>
//  0x3A BParameterArray<BParFX,8>
//  0x32 VoiceGroups
//  0x33 GroupList
//  0x34 ZoneList
// 0x3d FileNameListPreK1

use crate::{kontakt::Chunk, nks::BPatchMetaInfoHeader};

#[derive(Debug)]
pub struct Kon4 {
    pub chunks: Vec<Chunk>,
    pub meta_info: BPatchMetaInfoHeader,
}

// impl Kon4 {
//     pub fn program(&self) -> Result<Program, KontaktError> {}
//     pub fn filenamelist(&self) -> Result<FNTableImpl, KontaktError> {}
// }
