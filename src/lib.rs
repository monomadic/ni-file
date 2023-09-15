#![allow(dead_code)]
#![doc = include_str!("../doc/README.md")]

mod error;
pub use error::*;

// generic wrapper:
pub mod nifile; // simplified api for all supported filetypes

// containers:
pub mod file_container; // monoliths
pub mod nis; // nisound document // kontakt 4.2 preset
pub mod nks; // native instruments kontakt sound file format

// preset types:
pub mod fm8;
pub mod kontakt;

// utils:
pub(crate) mod byte_reader; // for reading bytestreams
pub mod deflate; // fastlz decompression
mod detect; // detect filetype
pub(crate) mod prelude;
pub(crate) mod read_bytes; // for reading bytestreams
pub(crate) mod string_reader;
pub(crate) mod utils; // various utils for logging etc

pub use nifile::NIFile;
pub use nis::Repository;
