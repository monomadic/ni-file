<p align="center">
  <img src="assets/banner.jpg" />
</p>

# Native Instruments File Format

Native Instruments file format support for rust.

Current support:

| Application   | Type     | Detect | Container | Preset Data |
| ------------- | -------- | ------ | --------- | ----------- |
| Kontakt 1     | Kon1     | ✅     | ❌        | ❌          |
| Kontakt 2     | NKS      | ✅     | ✅        | 🕒 ~20%     |
| Kontakt 4.22+ | NKS      | ✅     | ✅        | 🕒 ~50%     |
| Kontakt 5+    | NISD     | ✅     | ✅        | 🕒 ~30%     |
| Kontakt 5+    | Monolith | ✅     | ✅        | ❌          |
| FM8           | NISD     | ✅     | ✅        | 🕒 ~60%     |

Anyone who wants to join the effort, please join the telegram group at https://t.me/ni_file

I'm on telegram at `@deathdisco`.

## Usage

Please check the [docs](/doc/README.md) for the latest api changes and schematics.

## Installation

This is a library, but there are helper binaries in the examples folder:

- `ni-info` prints information about NI file formats.
- `ni-tree` prints the tree structure of NISD containers.
- `ni-extract` dumps internal preset data from NISD containers.
- `ni-convert` (coming soon) converts between formats

```bash
cargo install --path . --example ni-info
```

To just run the examples in place, try:

```bash
cargo run --example ni-info -- tests/data/nisound/file/**/*.nki
cargo run --example ni-info -- tests/data/nisound/file/**/*.nkm
```

## FAQ

Q: Can I extract .wav files from kontakt files?
A: Not **yet** but there is functionality in the code for extracting samples from monoliths.

Q: Can I extract any meaningful information besides library and file metadata from any preset types?
A: Yes! ZoneData and so on is extractable from some kontakt files.

Q: Will there be write support?
A: Eventually, but this is a low priority. Please get involved if you wish to see write support earlier.

Q: Are compressed samples/presets supported?
A: Yes, files mostly use FastLZ compression internally and decompression is supported by the library.
