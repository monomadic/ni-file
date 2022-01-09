# Native Instruments File Format

## Why

I don't really know. I don't like the way Native Instruments treats the music industry, locking down file formats and being shady. As a young musician I wanted to be expressive and found music a closed box, with almost all seemingly promising leads actually just trashy salesmen.

Music should be open and inclusive, like development. And it's fun to see how this stuff is put together. These sample libraries are now getting old and support is starting to dwindle on them.

I don't even really do music much any more so this project has become recreational. It may never see a proper release, as I don't think anyone really cares about these old formats much.

I don't plan currently to write NI formats, just read them. It would be fairly trivial with this reversing work to write this functionality, so PRs are welcome.

## Progress

This library is a work in progress.

- [x] Detect various NI filetypes
- [x] Extract compressed presets from NI Containers (most NI files)
- [x] Extract files from Kontakt Monoliths
- [ ] Unencrypted presets
  - [~] Kontakt 4/5 Unencrypted Presets
  - [~] FM8
  - [ ] Kontakt 2
- [ ] Encrypted presets
  
There is no real code quality at this point, but this will follow once the container format is more discovered.

## NIContainer File Schematic

First off, the container format (the most used NI format) is one ridiculous file format, I have no idea what NI were thinking.

File is made up of nested segments, very similar to a linked list, which have a header of 20 bytes like the following:

<size u64><magic [char;4]><id u32><unknown (always 1) u32>

The magic part is a char array denoted with 'hsin' tags / magic numbers. These tags are spelt backwards. For example

- `hsin` Native Instruments Start Header
- `DSIN` Native Instruments Start Data
- `4KIN` Native Instruments Kontakt 4
- `RTKR` ReaKToR
- `E8MF` FM8 E?

There are two major kinds of segments header segments (`hsin`) and data segments (`dsin`). Header segments have more information and nest data segments:

<size u64><magic "hsin"><id u32><unknown (always 1) u32>
<checksum [16-bytes]>
[data-segments]
<data>

Segments contain two parts: headers (which can nest other segments) and then the data payload:

<segment>[<child segments>]<data>

## Compressed Presets

The main preset is compressed with a custom [LZ77](https://en.wikipedia.org/wiki/LZ77_and_LZ78) variant. deflate.rs can deflate a segment. The segment will start as normal, but appears to embed another file (with its own data segments, compressed) as data in a `DSIN` (type 115).

IMPORTANT: the compression starts 11 bytes into the data slice, but you must provide an initial dictionary of `00`.

Note that checksums and file lengths for the file header are usually SKIPPED in kontakt, you can remove them entirely in some situations and the patch will still load. This also applies to `DSIN` tags. I think NI might have tried to make their code more efficient by directly reading offsets.

## Serialised Data

### Strings

Most strings are [pascal widestrings](https://wiki.lazarus.freepascal.org/Character_and_string_types#WideString) or [shortstrings](https://wiki.lazarus.freepascal.org/Character_and_string_types#ShortString).
