# Lib

## K4PatchLib::BPatchHeaderV2::CalcCurHashSum()
- this swaps byte order, so the md5 must be > 8bit segments in big endian

ia~CheckForEncryption
s 0x00007326
pdf
calls K4PL_ReadPatchHeaderValue
s sym.K4PL_ReadPatchHeaderValue_void__int__unsigned_int_
if ok calls sym._InitKIO else K4PL_ClosePatch

afl~PatchData
afl~ReadPatch

pdg @method.K4PatchLib::BPatchHeaderV42.IsMagicAndVersionValid___const
K4PatchLib::BPatchHeaderV42::IsMagicAndVersionValid() const
```rust
fn is_magic_and_version_valid(arg) -> bool {
  if arg[10] == 0xe69cc815 {
    if arg == 0xff107a54 || arg == 0x53438710 || arg == 0x1290a87f {
      return arg[2] == 0x110;
    }
  }
  false
}
```
note: 0x1290a87f found in dsin block of k4 files

# interesting huge function
s method.K4PL_DETAIL::InternalPatchData.ExtractHeader__

0x00000000 [0x000a61b0 - 0x000a63d0] NI::SOUND::Lib
0x00000000 [0x000a63e0 - 0x00209738] NI::SOUND::Bank
0x00000000 [0x000a6df0 - 0x00209768] NI::SOUND::Preset
K4PO::ChunkData
