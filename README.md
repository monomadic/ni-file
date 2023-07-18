# Native Instruments File Format

Native Instruments file format support for rust.

Current support:

| Application | Type | Detect | Container | Preset Data      |
| ----------- | ---- | ------ | --------- | ---------------- |
| Kontakt 1   | Kon1 |       |          |                 |
| Kontakt 2   | NKS  |       |          |                 |
| Kontakt 4   | NKS  |       |          | 󰔚 Zones, Samples |
| Kontakt 5+  | NISD |       |          | 󰔚 ~15%           |
| FM8         | NISD |       |          | 󰔚 ~50%           |

Anyone who wants to join the effort, please join the telegram group at https://t.me/ni_file

I'm on telegram at `@deathdisco`.

## Installation

This is a library, but several helper binaries will be included.

- `ni-info` prints information about NISD containers.
- `ni-tree` prints the tree structure of NISD containers.
- `ni-extract` (coming soon) dumps presets and samples from ni files
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
