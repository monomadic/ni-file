<p align="center">
  <img src="assets/banner.jpg" />
</p>

# Native Instruments File Format

Native Instruments file format support for rust. This library will serve as a reference implementation is the result of countless hours of painstaking reverse engineering and research. As this repository also serves as a research base, the implementation will shift drastically for a while and the code will be under a state of refactoring.

The tests will break and you may get (intentional) panics. Right now finding gaps in the spec is more important than developer comfort. Once a 0.1 release is published to crates.io, the api and build process will be more reliable.

Having said that, this library is already semi-useful; the various NI compression algorithms, generic containers and wrappers, and the entire family of Kontakt formats are nearing a competent level of support. Kontakt files can mostly be read to some extent, but not all data is fully mapped.

If you'd like to join the effort or just learn more about reverse engineering, please join the telegram group at https://t.me/ni_file I'm on telegram at `@deathdisco`. I also run the [rust-audio telegram group](https://t.me/+TrgB_G5z0Yl6WYat).

## Features

- ✅ **Kontakt v1** _100%: extract xml_
- ✅ **Kontakt v2** _90%: extract xml, no monolith support yet_
- ✅ **Kontakt v4.22+** _75%: NKS Container, Program, FileTable, Zones_
- ✅ **Kontakt v5-v7** _65%: NIS Container, Program, FileTable, Zones_
- ✅ **Kontakt FileContainer (Modern Monolith)**: _read support_
- ✅ **NCW Compression**: _read support - also available as a [cli utility](https://github.com/monomadic/ncw)_.
- 🕒 **Kontakt NKS Monolith**: _coming soon_

## Planned Features

- 🕒 FM8 _partial_
- ❌ WebAssembly _needs testing_
- ❌ All other NI formats...
- ❌ BigEndian file support (generally very very old presets... probably not worth bothering about)

## Usage

For the rust api check https://docs.rs/ni-file. For file schematics check the repository [docs](/doc/README.md) directory. There are also several [examples](/examples/) included which show more advanced usage.

You might also wish to check out my [working repository](https://github.com/monomadic/hexfiend-templates) of [hexfiend](https://hexfiend.com/) templates. This is where a lot of my work in reverse engineering file formats begins, and has some file information not present in this repository (and some other RE projects). If a file is not reading correctly with this library, the place to start looking is usually by having a set of these templates on hand and viewing the file with that.

## Installation

This is a library, but there are helper binaries in the examples folder:

- `ni-info` prints information about NI file formats.
- `ni-tree` prints the tree structure of NIS containers.
- `ni-extract` dumps internal preset data from NIS Containers, NKI Instruments and NIS FileContainers.
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

A: Creating music with audio software is a totally ruined experience because of vendor lock-in. As a kid, I always hoped it would change. Several decades later here we are, worse than ever. So one of my life goals is to forcibly reverse engineer every proprietary audio format out there. Don't like it? Tough shit.

Also, I am a reverse engineer at heart, it's fun, and this is a good cause.

---

Q: What is the purpose of this library?

A: First, to document and expose these file types so that artists can have actual control of their own work, and second, to allow anyone to convert out of proprietary formats into open source standards. There are other use cases for this software, such as file support in third party software, but those are not interesting to me and not the primary focus of this knowledge work.

---

Q: What can this library do?

A: It has been a long slog but finally various formats are starting to be supported. The focus is on Kontakt single instruments for now (all versions). You can currently decompress samples, extract a lot of information such as Key Zones and metadata. You could in theory write a SFZ conversion tool for example, but it would be a lossy conversion at the moment.

---

Q: Will there be write support?

A: Maybe, nothing is stopping anyone adding it but mostly I want to enable conversion to more open formats. Please get involved if you wish to see write support earlier. I will probably implement output for NKS Kontakt 4 as the format is simpler and NI haven't really innovated on Kontakt since 1992 anyway (unless you think drm and digital purchases stuffed into your music application a 'feature'). As for NISound writing, if there's a reason I'm sure it will happen.

---

Q: Can I help?

A: Please do! Join our [telegram group](https://t.me/ni_file) and we would be happy to answer any questions you have.
