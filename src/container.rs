/**
 * Container
 * high-level model for a ni-container
 */

#[derive(BinRead, Debug)]
pub struct Segment {
    fields: Vec<Field>,
    children: Segment,
    data: Vec<u8>,
}

#[derive(BinRead, Debug)]
pub struct HeaderChunk {
    pub length: u64,
    pub unknown_a: u32, // item deferred flag?

    #[br(assert(tag==['h','s','i','n']))]
    pub tag: [char; 4],
    pub id: u64,            // uuid?
    pub checksum: [u8; 16], // md5 of child section (including child chunk)

    pub data_len: u32,
    #[br(count = data_len, seek_before=std::io::SeekFrom::Current(-4))]
    pub data_chunk: Vec<u8>,

    pub current_index: u32,

    pub children_length: u32,
    #[br(count = children_length)]
    pub children: Vec<ChildChunk>,
}

#[derive(BinRead, Debug)]
pub struct ChildChunk {
    pub unknown_a: u32, // SUSPICIOUS NUMBER - COMPRESSION HAS HIGH VALUE
    pub tag: [char; 4],

    #[br(parse_with = SegmentType::binread)]
    pub id: SegmentType,
    pub chunk: HeaderChunk,
}
