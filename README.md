# Native Instruments File Format

UPDATE: The container format is fairly well understood now (enough to read blocks and extract files and presets), and compressed internal presets are spat out on all files of this type. From here, deciphering individual presets is much more straightforward.

Anyone who wants to do this with me, please get in touch. I'm on telegram at @deathdisco

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

First off, the container format (the most used NI format) is one ridiculous file format, my best guess is that it is built for fast reading, rather than something easy or efficient to parse (because it's terribly inefficient). It took many many hours/days of staring into a hex editor to understand.

The file is made up of nested segments, very similar to a linked list. There are two major kinds of segments header segments (`hsin`) and data segments (`dsin`). Header segments have more information and nest data segments:

```xml
<size u64>
<hsin_header (20 bytes)>
<checksum (16 bytes)>
[data-segments]
<data>
```

The magic part is a char array denoted with 'hsin' tags / magic numbers. These tags are spelt backwards. For example

- `hsin` Native Instruments Start Header
- `DSIN` Native Instruments Start Data
- `4KIN` Native Instruments Kontakt 4
- `RTKR` ReaKToR
- `E8MF` FM8 E?

### HSIN Header (20 bytes)

```xml
<unknown u32><unknown u32><magic 'hsin'><id u32><unknown u32 (always 1)>
```

### DSIN Header (20 bytes)

Segments contain two parts: headers (which can nest other segments) and then the data payload:

```xml
<size u32><unknown u32><magic 'hsin'><id u32><unknown u32 (sometimes size? offset maybe?)>
[<child segments>]
<data>
```

## Compressed Presets

The main preset is compressed with a custom [LZ77](https://en.wikipedia.org/wiki/LZ77_and_LZ78) variant. deflate.rs can deflate a segment. The segment will start as normal, but appears to embed another file (with its own data segments, compressed) as data in a `DSIN` (type 115).

IMPORTANT: the compression starts 11 bytes into the data slice, but you must provide an initial dictionary of `00`.

Note that checksums and file lengths for the file header are usually SKIPPED in kontakt, you can remove them entirely in some situations and the patch will still load. This also applies to `DSIN` tags. I think NI might have tried to make their code more efficient by directly reading offsets.

## Serialised Data

### Strings

Most strings are [pascal widestrings](https://wiki.lazarus.freepascal.org/Character_and_string_types#WideString) or [shortstrings](https://wiki.lazarus.freepascal.org/Character_and_string_types#ShortString).

## Running

``` bash
cargo +nightly run -- test-data/deflated/002-fm7.nfm8.deflated
```
