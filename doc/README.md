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

Repositories are embedded hierarchical chunks of data known as `Item`s. Items consist of an `ItemHeader`, an `ItemFrameStack` and finally child `Item`s.

This is the basic structure:

- `Repository`
    - `Item`
        - `ItemHeader` 20 bytes ('hsin') NI::SOUND::ItemHeader::write(NI::GP::Stream&)
            - @ u64 FrameSize
            - @ u32
            - @ u32 DomainID 0x6e697368 'hsin'
            - @ u32 ItemID
            - @ u32
            - @ uuid method.NI::SOUND::ItemUuid.write_NI::GP::Stream__const
        - `ItemFrameStack`
            - `ItemFrame` (Size, DomainID, ItemID)
                - @ u64 FrameSize +0
                - @ u32 DomainID 0x4e495344 "DSIN" +0x8
                - @ u32 ItemID +0xc(12)
                - @ u32 Version +0x10(16)


### `ItemHeader`

This is the header of an `Item`, listing its size, magic number, and flags such as defered status. It does not carry type information or anything interesting. It has a kind of magic number / string, `hsin`,

- `hsin` Native Instruments Sound Header

![chunks](../assets/chunks.png)

### `ItemFrameStack`

This is the data portion of the `Item`, `ItemFrameStack` contains `ItemFrame`s organised in a stack structure. The topmost stack denotes the items type. Types are determined by `ItemID` and `DomainID`.

### `DomainID` and `ItemID`

Domains are groups of item types, the most common of which is `dsin`, and most items you find will be in this domain. Each particular file type will have specific domains, here are some I have found:

- `DSIN` Native Instruments Sound Data
- `4KIN` Native Instruments Kontakt 4
- `RTKR` ReaKToR
- `E8MF` FM8 Ensemble

![data](../assets/data.png)



### Properties

Within some `ItemFrame`s are properties, some are compressed though there are several types of properties.

### Compressed Presets

The main preset is compressed with a variant of [LZ77](https://en.wikipedia.org/wiki/LZ77_and_LZ78) called FastLZ. deflate.rs can deflate a segment. It is the property that is compressed, not the `Frame`.

https://github.com/ariya/FastLZ
https://crates.io/crates/fastlz not native, rust bindings

### Strings

Most strings are [pascal widestrings](https://wiki.lazarus.freepascal.org/Character_and_string_types#WideString) or [shortstrings](https://wiki.lazarus.freepascal.org/Character_and_string_types#ShortString).

## Tests

``` bash
cargo test
```
