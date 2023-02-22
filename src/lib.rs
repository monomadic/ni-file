//!
//!  NI-FILE
//!  Native Instruments file formats
//!

// #![warn(clippy::all)]
// #![warn(missing_docs)]
// #![warn(clippy::style)]

// TODO: remove
#![allow(dead_code)]

#[macro_use]
extern crate log;

pub mod error;
pub mod prelude;

mod detect; // detect filetype
pub mod repository; // read ni repositories

// utils
pub mod cb; // control byte
pub mod decompress; // fastlz lib
pub mod deflate; // decompress

pub(crate) mod read_bytes; // for reading bytestreams
pub(crate) mod utils; // various utils for logging etc

pub use crate::detect::NIFileType;
