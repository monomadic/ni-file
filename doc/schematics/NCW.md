# NCW Compression

## Basic Schematic

- sections:

  - header
    - offset 0
    - usually 120 bytes
  - table of block offsets
    - offset 120
  - blocks

- each block is 512 samples of uncompressed audio
- compression is bit truncating
- sometimes channels are stored as MID+SIDE vs stereo (LEFT+RIGHT)
