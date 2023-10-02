// Kon4 Schema:
//

use crate::{kontakt::Chunk, nks::BPatchMetaInfoHeader};

#[derive(Debug)]
pub struct Kon4 {
    pub chunks: Vec<Chunk>,
    pub meta_info: BPatchMetaInfoHeader,
}
