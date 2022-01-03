# Native Instruments File Format

## Why

I don't really know. I don't like the way Native Instruments treats the music industry, locking down file formats and being shady. As a young musician I wanted to be expressive and found music a closed box, with almost all seemingly promising leads actually just trashy salesmen.

Music should be open and inclusive, like development. And it's fun to see how this stuff is put together. These sample libraries are now getting old and support is starting to dwindle on them.

I don't even really do music much any more so this project has become recreational. It may never see a proper release, as I don't think anyone really cares about these old formats much.

## Progress

This library is a work in progress. It can read files and do a few other things so far. It has taken many many hours staring at a hex editor to get to this point. Any help would be appreciated, and if you'd like to request anything in particular, please make [contact](mailto:themonomadic@protonmail.com)

There is no real code quality at this point, but this will follow once the container format is more  reversed.

- [x] NI file containers (many unknown fields, but fully extracts data segments)
- [x] Extract compressed presets
- [x] Version
- [ ] Metadata - partially working
- [x] Extract Kontakt Monoliths
- [ ] Kontakt 2 (some progress)
- [ ] Kontakt 4/5
  - [ ] Sample List
- [ ] Kontakt 5 Encrypted

## NIContainer File Schematic

The file format is tricky at first, took me a while to work it out. It is used for most NI apps (but not monoliths).

File is made up of nested segments, denoted with 'hsin' tags / magic numbers. These tags are spelt backwards. For example

- `hsin` Native Instruments Start Header
- `DSIN` Native Instruments Start Data
- `4KIN` Native Instruments Kontakt 4
- `RTKR` ReaKToR
- `E8MF` FM8 E?

The basic file consists of segments listed below in this format:

<HSIN>
    <DSIN>
    <HSIN>
        <DSIN></HSIN>
    </HSIN>
</HSIN>

Data segments (`dsin`) can actually be app-specific tags like `4kin` etc.

### HSIN: Native Instruments Start Header

<SEGMENT_SIZE u64>
<CHILDREN? u64>
<MAGIC (eg. "hsin" etc)>
<UNKNOWN u64 (data segments?)>
<CHECKSUM 16-bytes, probably md5>
[<DSIN> ...]



## Compressed Presets

The main preset is compressed with a custom [LZ77](https://en.wikipedia.org/wiki/LZ77_and_LZ78) variant. deflate.rs can deflate a segment. The segment will start as normal, but appears to embed another file (with its own data segments, compressed) as data in a `DSIN` (type 115).

IMPORTANT: the compression starts 11 bytes into the data slice, but you must provide an initial dictionary of `00`.

Note that checksums and file lengths for the file header are usually SKIPPED in kontakt, you can remove them entirely in some situations and the patch will still load. This also applies to `DSIN` tags. I think NI might have tried to make their code more efficient by directly reading offsets.

## Serialised Data

### Strings

Most strings are [pascal widestrings](https://wiki.lazarus.freepascal.org/Character_and_string_types#WideString) or [shortstrings](https://wiki.lazarus.freepascal.org/Character_and_string_types#ShortString).
