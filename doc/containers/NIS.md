# Native Instruments Sound Containers

The most modern repository/container format for almost all types of NI files.

Each [`NISound`] is like a mini database of sorts, and you can read these repositories with low
level structs (embedded [`Item`](crate::nisound::Item)s) or use high-level structs such as [`NISound`]. It is
recommended and much easier to use the latter unless you are dealing with filetypes still
undocumented by the library.

```
use ni_file::{NIFileType, NISound, NIMonolith};

let file = std::fs::File("tests/data/nisound/file/fm8/1.2.0.1010/001-fm7.nfm8").unwrap();

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
