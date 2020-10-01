# Native Instruments File Format

## Progress

This library is a work in progress. It can read files and do a few other things so far. Any help would be appreciated.

- [x] Read NI file containers
- [x] Read type (kontakt, fm8, etc)
- [x] Read version number
- [x] Decompress compressed segments
- [ ] Full support for NI file containers (all fields understood and documented)
- [ ] ... everything else?

## File Schematic

### Block Format

File is made up of nested blocks, denoted with 'hsin' tags / magic numbers.

#### HSIN - Header Section In
``` xml
<LENGTH_OF_SEGMENT: le_u64>
<HEADER_TAG: "hsin" / 4-bytes>
<CHECKSUM 16-bytes>
<DATA_SEGMENT_LENGTH le_u64>
<DATA_SEGMENT {DATA_SEGMENT_LENGTH}-bytes>
<NUMBER_OF_CHILDREN le_u32> // usually 0
<CHILD_TYPEID le_u32>
<EMBEDDED_SEGMENTS (remaining data)>
```

For example, `[2557][1]["hsin"][1]["ÅÈ¿K 0ÉGlUG"][...][1][1]["4KIN", 3]`

The first block length in the file will be the entire file size, as it represents one block and other hsin blocks are embedded within.

### DSIN - Data Structure In

DSIN blocks act as maps or slice indexes. The first DSIN is usually the length of the whole data chunk, minus footer data. Each DSIN is embedded inside its parent data. Like inception. Only dumber.

Format:
``` xml
<LENGTH_OF_SEGMENT: le_u64>
<HEADER_TAG: "DSIN" | "4KIN" | "RTKR" | etc / 4-bytes>
<TYPE: le_u32>
<UNKNOWN: le_u32> // always 1
<CHILD_DSIN_LENGTH: le_u64>
<CHILD_DSIN {CHILD_DSIN_LENGTH}-bytes>
<DATA (remaining data)>
```

Footer data

## Compressed segments

The main preset is compressed with a custom LZ77 variant. deflate.rs can deflate a segment. The segment will start as normal, but appears to embed another file (with its own data segments, compressed) as data in a DSIN (type 115).

IMPORTANT: the compression starts 11 bytes into the data slice, but you must provide an initial dictionary of 00.

Note that checksums and file lengths for the file header are usually SKIPPED in kontakt, you can remove them entirely in some situations and the patch will still load. This also applies to DSIN tags. I think NI might have tried to make their code more efficient by directly reading offsets.
