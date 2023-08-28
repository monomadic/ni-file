# Native Instruments file formats.

It might be worth reading [TERMINOLOGY.md](TERMINOLOGY.md) which describes terms used throughout the code and this documentation.

## Reading files

There is some overlap between the many file formats NI has used across the decades. In general, one or more presets are wrapped in a container format.

Kontakt originally had its own container format, called NKS (Native Instruments Kontakt Sound), and embedded inside was a compressed XML document with the actual preset data. Eventually the preset data was replaced with a custom RIFF-like binary chunk document.

Eventually all NI products used one single format, NIS (Native Instruments Sound), sometimes also referred to as NISound. This is a a slightly more sturdy (though still terrible) binary chunk format similar to EBML. Kontakt still used the same chunk format as previously, without the NKS container.

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
