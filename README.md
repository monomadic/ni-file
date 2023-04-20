# Native Instruments File Format

Native Instruments file format support for rust. Very basic support at the moment, mostly surrounding the container format (called a `NISound` document).

Anyone who wants to join the effort, please join the telegram group at https://t.me/ni_file

I'm on telegram at `@deathdisco`.

## Progress

After several years doing this part time, the container format is around 80% reversed, not all of this is reflected in the code. Currently you can:

- [x] read container metadata (NISound version, App type etc)
- [x] search / traverse container hierarchy with inspection
- [x] access inner presets (the actual raw preset data of each app)

More is coming very shortly, please check the examples directory for usage.

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
A: Not **yet** but this is coming very very soon. The container format took years to reverse engineer part-time but that was most of the tricky work done. This work also opens the door for other reversers to document simpler internal preset structures.

Q: Will there be write support?
A: I don't really care about keeping people inside the NI ecosystem, just helping them escape it, so probably not unless someone PRs it.

Q: Are compressed samples/presets supported?
A: Yes, files mostly use FastLZ compression internally and tdecompression is supported by the library.

Q: Is decryption supported?
A: No, but this will be eventually implemented and supported through legal means.

## Special Thanks

To Native Instruments, for being a driving force in monopolising the audio software industry, helping to make it so proprietary that I learned how to reverse engineer to open up your formats to everyone. I learned so much due to your propensity for evil. As anyone who has ever delved into audio software knows, proprietary formats have killed the experience ever since it first existed. So many projects destroyed midway due to 'compatibility issues' and whatnot. If you want to make samplers or dildos best of luck I don't care, but you don't own what people make with your software, including the patch metadata itself.

Fair use exists for a reason, and this is it.
