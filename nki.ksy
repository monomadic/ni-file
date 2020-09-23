meta:
  id: nki
  file-extension: nki
  endian: le
seq:
  - id: chunk_size
    type: u8
  - id: chunk_data
    type: header_chunk
    size: chunk_size - 8
types:
  data_chunk:
    seq:
      - id: data_tag
        type: str
        size: 4
        encoding: UTF-8
      - id: data_type_id
        type: u4
      - id: unk
        type: u4
      - id: next_data_chunk_size
        type: u4
      - id: is_end
        type: u4
    instances:
      data:
        pos: 142 - next_data_chunk_size
        size: next_data_chunk_size

  header_chunk:
    seq:
      - id: unknown1
        type: u4
      - id: tag
        type: str
        size: 4
        encoding: UTF-8
      - id: unknown2
        type: u8
      - id: checksum
        size: 16
      - id: data_chunk_size
        type: u8
      - id: datachunk
        type: data_chunk
        # size: data_chunk_size - 8
        repeat: expr
        repeat-expr: 3
