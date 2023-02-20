# Native Instruments File Format

Native Instruments file format support for rust. Very basic support at the moment, mostly surrounding the container format (called a Repository). Knowledge is all reverse engineered.

Anyone who wants to do this with me, please get in touch. I'm on telegram at @deathdisco.

## Progress

This library is a work in progress.

- [x] Detect various NI filetypes
- [x] Extract compressed presets from NI Containers (most NI files)
- [x] Extract files from Kontakt Monoliths
- [ ] Unencrypted preset extraction
  - [~] Kontakt 4/5 Unencrypted Presets
  - [~] FM8
  - [ ] Kontakt 2
- [ ] Encrypted presets

## Repository File Schematic

Repositories are embedded hierarchical chunks of data known as Items.

Each file is made up of nested segments. There are two major kinds of segments header segments (`hsin`) and data segments (`dsin`). Header segments have more information and nest data segments. Here's a basic example colored with high level chunks.

![chunks](assets/chunks.png)

The magic part is a char array denoted with 'hsin' tags / magic numbers. These tags are spelt backwards. For example

- `hsin` Native Instruments Sound Header
- `DSIN` Native Instruments Sound Data
- `4KIN` Native Instruments Kontakt 4
- `RTKR` ReaKToR
- `E8MF` FM8 Ensemble

Another way to understand this structure is as follows:
- `Repository`
    - `Item`
        - `ItemHeader` 20 bytes ('hsin') NI::SOUND::ItemHeader::write(NI::GP::Stream&)
            - @ u64 FrameSize
            - @ u32
            - @ u32 DomainID 0x6e697368 'hsin'
            - @ u32
            - @ u32
            - @ uuid method.NI::SOUND::ItemUuid.write_NI::GP::Stream__const
        - `ItemFrameStack`
            - `ItemFrame` (Size, DomainID, ItemID)
                - @ u64 FrameSize +0
                - @ u32 DomainID 0x4e495344 "DSIN" +0x8
                - @ u32 ItemID +0xc(12)
                - @ u32 Version +0x10(16)

### Frames

`Frames` are data fields, and are grouped together in a `StackFrame`.

![data](assets/data.png)

### Properties

Within a `Frame` are properties, some are compressed though there are several types of properties.

### Compressed Presets

The main preset is compressed with a custom [LZ77](https://en.wikipedia.org/wiki/LZ77_and_LZ78) variant. deflate.rs can deflate a segment. It is the property that is compressed, not the `Frame`.

IMPORTANT: the compression starts 11 bytes into the data slice (depending on the property), but you must provide an initial dictionary of `00`.

### Strings

Most strings are [pascal widestrings](https://wiki.lazarus.freepascal.org/Character_and_string_types#WideString) or [shortstrings](https://wiki.lazarus.freepascal.org/Character_and_string_types#ShortString).

## Tests

``` bash
cargo test
```
