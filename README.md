# Native Instruments File Format

Native Instruments file format support for rust. Very basic support at the moment, mostly surrounding the container format (called a `NISound` document).

Anyone who wants to join the effort, please join the telegram group at https://t.me/ni_file

I'm on telegram at `@deathdisco`.

## Installation

This is a library, but several helper binaries will be included.

- `ni-info` prints information about NISD containers.
- `ni-tree` prints the tree structure of NISD containers.
- `ni-extract` (coming soon) dumps presets and samples from ni files
- `ni-convert` (coming soon) converts between formats

```
cargo install --path . --example ni-info
```

To just run the examples in place, try:
```
cargo run --example ni-info -- tests/data/nisound/file/**/*.nki
cargo run --example ni-info -- tests/data/nisound/file/**/*.nkm
```

## FAQ

Q: Can I extract .wav files from kontakt files?
A: Not **yet** but there is functionality in the code for extracting samples from monoliths.

Q: Can I extract any meaningful information besides library and file metadata from any preset types?
A: Yes! ZoneData and so on is extractable from kontakt files.

Q: Will there be write support?
A: I don't really care about keeping people inside the NI ecosystem, just helping them escape it, so probably not unless someone PRs it.

Q: Are compressed samples/presets supported?
A: Yes, files mostly use FastLZ compression internally and tdecompression is supported by the library.

Q: Is decryption supported?
A: No, but this will be eventually implemented and supported through legal means.
