#![allow(dead_code)]
#![doc = include_str!("../doc/README.md")]

mod error;
pub use error::*;

// generic wrapper:
pub mod nifile; // simplified api for all supported filetypes

pub use nifile::NIFile;

// containers:
pub mod file_container; // monoliths
pub mod nis; // nisound document // kontakt 4.2 preset
pub mod nks; // native instruments kontakt sound file format

pub use nis::Repository;

// app domains:
pub mod fm8;
pub mod kontakt;

// utils:
mod byte_reader; // for reading bytestreams
mod detect; // detect filetype
mod prelude;
mod read_bytes; // for reading bytestreams
mod string_reader;
mod utils; // various utils for logging etc
