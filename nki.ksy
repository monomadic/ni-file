meta:
  id: nki
  file-extension: nki
  endian: le
seq:
  - id: length
    type: u8
  - id: chunk_data
    type: header_chunk
    size: length - 8

types:
  data_chunk:
    seq:
      - id: tag
        type: str
        size: 4
        encoding: UTF-8
      - id: type
        type: u4
      - id: unknown
        type: u4
      - id: datachunk_length
        type: u8
        if: type != 1
      - id: datachunk
        type: data_chunk
        if: type != 1
        size: datachunk_length - 8
      - id: values
        size-eos: true

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
      - id: datachunk_length
        type: u8
      - id: datachunk
        type: data_chunk
        size: datachunk_length - 8
      - id: no_compression
        type: u4
      - id: children
        type: u4
      - id: extra_tag
        size: 12
        if: children > 0 and no_compression == 1
      - id: length
        type: u8
        if: children > 0 and no_compression == 1
      - id: chunk_data
        type: header_chunk
        size: length - 8
        if: children > 0 and no_compression == 1
