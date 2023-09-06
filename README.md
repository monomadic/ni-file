<p align="center">
  <img src="assets/banner.jpg" />
</p>

# Native Instruments File Format

Native Instruments file format support for rust. This library is the result of hundreds of hours of painstaking research.

## Features

- 🕒 Kontakt 1/2 _partial_
- 🕒 FM8 _partial_
- ✅ Kontakt 4.22+ _partial: NKS Container, Program, FileTable, Zones_
- ✅ Kontakt 5-7 _partial: NIS Container, Program, FileTable, Zones_
- 🕒 NKW Compression _partial: 16-bit mono supported, other formats to follow_

## Planned Features

- ❌ FileContainer / Monolith _detection only_
- ❌ WebAssembly / nostd _internal lz77 compression temporarily retired, needs refactoring to remove zlib dependency_
- ❌ All other NI formats...

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

Q: Why are you doing this?
A: One of my life goals is to break the proprietary nature of audio software by reverse engineering every single proprietary format out there, under fair use, and Native Instruments formats are particularly egregious, locked in, and monopolistic, so they are first. If you or NI don't like it, eat me I don't care. If your sole business proposition is vendor lock in, you deserve a smart person with unlimited time on your ass. This is legal and fair use and even if it wasn't I'd still release this stuff, as long as I have power and laptop, because I believe in doing so. The audio software industry should be as good as every other software industry, but it isn't because open source isn't given a chance. This project is the culmination of literally hundreds of hours of my life, but the skills I have gained will make the next project take a tenth the time.

Q: Can I extract .wav files from kontakt monolith files?
A: Not **yet** but there is functionality in the code for extracting samples from monoliths. This is not a difficult task, PRs welcome.

Q: Can I extract any meaningful information besides library and file metadata from any preset types?
A: Yes! Program Info, ZoneData, and FileTables are extractable from most Kontakt files.

Q: Will there be write support?
A: Eventually, but this is a low priority. Please get involved if you wish to see write support earlier.

Q: Are compressed samples/presets supported?
A: Yes, files mostly use FastLZ compression internally and decompression is supported by the library.
