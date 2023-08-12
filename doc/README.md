# Native Instruments file formats.

It might be worth reading [TERMINOLOGY.md](TERMINOLOGY.md) which describes terms used throughout the code and this documentation.

## Reading files

You first need to know what filetype you are dealing with, so use
[`NIFileType::detect`]. Most NI presets these days are
[`NIFileType::NISound`], unless they are a bundle of files (Kontact instruments and samples),
where they could be a [`NIFileType::NIMonolith`].

### NISound Containers

Each [`NISound`] is like a mini database of sorts, and you can read these repositories with low
level structs (embedded [`Item`](crate::nisound::Item)s) or use high-level structs such as [`NISound`]. It is
recommended and much easier to use the latter unless you are dealing with filetypes still
undocumented by the library.

```
use ni_file::{NIFileType, NISound, NIMonolith};

let file = std::fs::read("tests/data/nisound/file/fm8/1.2.0.1010/001-fm7.nfm8").unwrap();

match NIFileType::detect(&file) {
    NIFileType::NISound => {
        let container = NISound::read(file.as_slice()).unwrap();
    }
    NIFileType::NKSLE | NIFileType::NKSBE => {
        let kontakt = NKSFile::read(file.as_slice())?;
    }
    // ...
    _ => unimplemented!(),
}
```
