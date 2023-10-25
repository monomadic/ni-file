#[derive(Debug)]
pub struct NKRItem {
    header: NKRHeader,
    chunk: NKRChunk,
}

#[derive(Debug)]
pub struct NKRHeader {
    magic: u32,
    version: u16,
    a: u32,
    b: u32,
    num_items: u32,
    c: u32,
}

#[derive(Debug)]
pub struct NKRChunk {
    length: u16,
    ref_ptr: u32,
    chunk_type: u16,
    data: Vec<u8>,
}
