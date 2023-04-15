//! Kontakt42 Preset File
//!

pub struct Kontakt42;

/// The header of a [`Kontakt42`] preset.
///
/// | Offset | Length | Type     | Meaning                     | Default    | Other                                    |
/// |--------|--------|----------|-----------------------------|------------|------------------------------------------|
/// | 0      | 4      | uint32_t | magic                       | 0x1290A87F |                                          |
/// | 4      | 4      | uint32_t | zLibLength                  | 0          |                                          |
/// | 8      | 2      | uint16_t | fileVersion (must be 0x110) | 0x110      |                                          |
/// | 10     | 4      | uint32_t | version                     | 0xea37631a |                                          |
/// | 14     | 2      | uint16_t | type                        | 1 (nki)    | 0=nkm, 1=nki, 2=nkb, 3=nkp, 4=nkg, nkz=5 |
/// | 16     | 4      | uint32_t | appVersion                  | 0x50500ff  |                                          |
/// | 20     | 4      | uint32_t | appSignature                | 0x4b34504c |                                          |
/// | 24     | 4      | time_t   | creationTime                |            |                                          |
/// | ..     |        |          |                             |            |                                          |
/// | 178    | 4      | uint32_t | appSVNRev                   |            |                                          |
pub struct BPatchHeaderV42;
