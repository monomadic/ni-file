#![allow(dead_code)]
#![doc = include_str!("../doc/README.md")]

#[macro_use]
extern crate log;

mod error;
pub use error::*;

// preset types:
pub mod fm8;
pub mod kontakt;

// containers:
pub mod file_container; // monoliths
pub mod nisound; // nisound document // kontakt 4.2 preset
pub mod nks; // native instruments kontakt sound file format

// wrappers:
pub mod nifile; // generic, simplified api for supported filetypes

// utils:
pub mod deflate; // fastlz decompression
mod detect; // detect filetype
pub(crate) mod prelude;
pub(crate) mod read_bytes; // for reading bytestreams
pub(crate) mod utils; // various utils for logging etc

pub use detect::NIFileType;
pub use nisound::Repository;
