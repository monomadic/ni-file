<p align="center">
  <img src="assets/banner.jpg" />
</p>

# Native Instruments File Format

Native Instruments file format support for rust. This library is the result of hundreds of hours of painstaking research.

## Features

- ✅ NCW Compression
- 🕒 Kontakt v1 _partial_
- 🕒 Kontakt v2 _partial_
- ✅ Kontakt 4.22+ _partial: NKS Container, Program, FileTable, Zones_
- ✅ Kontakt 5-7 _partial: NIS Container, Program, FileTable, Zones_
- 🕒 FM8 _partial_

## Planned Features

- ❌ FileContainer / Monolith _detection only_
- ❌ WebAssembly / nostd _internal lz77 compression temporarily retired, needs refactoring to remove zlib dependency_
- ❌ All other NI formats...

Anyone who wants to join the effort, please join the telegram group at https://t.me/ni_file

I'm on telegram at `@deathdisco`.

## Usage

Please check the [docs](/doc/README.md) for the latest api changes and schematics.

You might also wish to check out the [working repository](https://github.com/monomadic/hexfiend-templates) I use for [hexfiend](https://hexfiend.com/). This is where a lot of my work in reverse engineering for file formats, and has some file information not present in this repository (and some other RE projects). If a file is not reading correctly with this library, the place to start looking is usually by having a set of these templates on hand and viewing the file with that.

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

Q: Why are you doing this?

A: The audio software industry is ruined because of vendor lockin. As a kid, I always hoped it would change. Several decades later here we are, worse than ever. So one of my life goals is to forcibly reverse engineer every single proprietary standard out there. Don't like it? Tough shit.

Also, I am a reverse engineer at heart, it's fun, and this is a good cause.

---

Q: Can I extract embedded files from monolith containers?

A: Not **yet** but there is functionality in the code for extracting samples from monoliths. This is not a difficult task, PRs welcome.

---

Q: Can I extract any meaningful information besides library and file metadata from any preset types?

A: Yes! Program Info, ZoneData, and FileTables are extractable from most Kontakt files.

---

Q: Will there be write support?

A: Maybe, I doubt it, nothing is stopping anyone adding it but mostly I want to enable conversion to more open formats. Please get involved if you wish to see write support earlier.

---

Q: Are compressed samples/presets supported?

A: Yes, files mostly use FastLZ compression internally and decompression is supported by the library. Samples use NCW compression, which is fully supported.
