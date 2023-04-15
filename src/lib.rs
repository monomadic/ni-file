//!
//!  Native Instruments file formats.
//!
//!  Special thanks to Native Instruments for keeping their file formats proprietary all these
//!  years. Your user-antagonistic behavior pushed me day after day to reverse engineer your file
//!  formats and I learned so much.
//!
//!  Cheers bros. ✌️
//!
//!  # Understanding the NI development ecosystem
//!
//!  It might be worth reading [TERMINOLOGY.md](https://github.com/monomadic/ni-file/tree/master/doc/TERMINOLOGY.md)
//!  which describes terms used throughout the code and this documentation.
//!
//!  # Reading files
//!
//!  You first need to know what filetype you are dealing with, so use
//!  [`NIFileType::detect`]. Most NI presets these days are
//!  [`NIFileType::NISound`], unless they are a bundle of files (Kontact instruments and samples),
//!  where they could be a [`NIFileType::NIMonolith`].
//!
//!  ## NISound Containers
//!
//!  Each [`NISound`] is like a mini database of sorts, and you can read these repositories with low
//!  level structs (embedded [`Item`](crate::nisound::Item)s) or use high-level structs such as [`NISound`]. It is
//!  recommended and much easier to use the latter unless you are dealing with filetypes still
//!  undocumented by the library.
//!
//! ```
//! use ni_file::{NIFileType, NISound, NIMonolith};
//!
//! let file = std::fs::read("tests/data/nisound/file/fm8/1.2.0.1010/001-fm7.nfm8").unwrap();
//!
//! match NIFileType::detect(&file) {
//!     NIFileType::NISound => {
//!         let container = NISound::read(file.as_slice()).unwrap();
//!     }
//!     NIFileType::NIMonolith => {
//!         let monolith = NIMonolith::read(file.as_slice()).unwrap();
//!     }
//!     // ...
//!     _ => unimplemented!(),
//! }
//! ```
//!
//!  ## Other presets
//!
//!  Older preset types are much simpler flat structures. These are collected together in the
//!  [`crate::preset`] module.
//!

#![allow(dead_code)]

#[macro_use]
extern crate log;

mod error;
pub use error::*;

mod detect; // detect filetype
mod monolith; // monolith / FileContainer
pub mod nisound; // nisound repositories
pub mod preset; // older NI preset types

pub(crate) mod cb; // control byte
pub(crate) mod decompress; // fastlz lib
pub(crate) mod deflate; // decompress
pub(crate) mod prelude;
pub(crate) mod read_bytes; // for reading bytestreams
pub(crate) mod utils; // various utils for logging etc

pub use detect::NIFileType;
pub use monolith::NIMonolith;
pub use nisound::NISound;
