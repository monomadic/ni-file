use super::objects::BPatchHeaderV42;

#[derive(Debug)]
pub struct KontaktPatch {
    pub header: BPatchHeaderV42,
    pub data: Vec<u8>,
    // pub sound_info: SoundInfoItem,
}
