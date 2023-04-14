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
//!  # Detecting filetypes
//!
//!  You first need to know what filetype you are dealing with, so use
//!  [NIFileType::detect](crate::NIFileType::detect).
//!

// #![warn(clippy::all)]
// #![warn(missing_docs)]
// #![warn(clippy::style)]

// TODO: remove
#![allow(dead_code)]

#[macro_use]
extern crate log;

mod error;
pub mod prelude;

mod detect; // detect filetype
mod monolith; // monolith / FileContainer
mod nisound; // nisound repositories
mod preset; // older NI preset types

pub(crate) mod cb; // control byte
pub(crate) mod decompress; // fastlz lib
pub(crate) mod deflate; // decompress
pub(crate) mod read_bytes; // for reading bytestreams
pub(crate) mod utils; // various utils for logging etc

pub use detect::NIFileType;
pub use read_bytes::*;

// NIRepository
pub use nisound::{item::Item, items, ItemID, NIContainer, PresetChunkItem};

pub use monolith::NIMonolith;
