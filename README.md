# Native Instruments File Format

## Progress

This library is a work in progress. It can read files and do a few other things so far. It has taken many many hours staring at a hex editor to get to this point. Any help would be appreciated, and if you'd like to request anything in particular, I'd love a [donation](#donations) :)

There is no real code quality at this point, but this will follow once the container format is 100% reversed.

- [x] Parse NI file containers
- [x] Extract compressed preset data (kontakt, fm8, etc)
- [x] Parse version block
- [x] Parse metadata / library block (preset name, author, bank, etc)
- [ ] Full support for NI file containers (all fields fully understood and documented)
- [ ] Parse presets
    - [ ] FM8 (Partial) (.nfm8)
    - [ ] Kontakt 4/5 (.nki)
    - [ ] Reaktor (.ens)
    - [ ] Massive (.nmsv)
- [ ] Write presets?
    - [ ] Understand checksums
- [ ] ... everything else?

## File Schematic

This is one ugly file format. If anyone recognises this kind of schematic (especially the way data blocks are stored), PLEASE get in contact with me and let me know. I have set up a gitter at [https://gitter.im/ni-file](https://gitter.im/ni-file).

### Block Format

File is made up of nested blocks, denoted with 'hsin' tags / magic numbers. These tags are spelt backwards. Some of the blocks are:

- `hsin` Native Instruments Start Header
- `DSIN` Native Instruments Start Data
- `4KIN` Native Instruments Kontakt 4
- `RTKR` ReaKToR
- `E8MF` FM8 E?

#### HSIN (NISH) - Native Instruments Start Header

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

The first block length in the file will be the entire file size, as it represents one block and other hsin blocks are embedded within.

#### DSIN (NISD) - Native Instruments Start Data

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

## Compressed Presets

The main preset is compressed with a custom [LZ77](https://en.wikipedia.org/wiki/LZ77_and_LZ78) variant. deflate.rs can deflate a segment. The segment will start as normal, but appears to embed another file (with its own data segments, compressed) as data in a `DSIN` (type 115).

IMPORTANT: the compression starts 11 bytes into the data slice, but you must provide an initial dictionary of `00`.

Note that checksums and file lengths for the file header are usually SKIPPED in kontakt, you can remove them entirely in some situations and the patch will still load. This also applies to `DSIN` tags. I think NI might have tried to make their code more efficient by directly reading offsets.

## Donations

- BTC: (I'll post an address soon, please get in touch if you prefer btc)
- ETH: monomadic.eth / `0xd86De8Bf49e2f10341e2fB62ebCb81f286e96f1A`

000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f
