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
mod monolith;
mod repository; // read ni repositories

// utils
pub(crate) mod cb; // control byte
pub(crate) mod decompress; // fastlz lib
pub(crate) mod deflate; // decompress

pub(crate) mod read_bytes; // for reading bytestreams
pub(crate) mod utils; // various utils for logging etc

pub use detect::NIFileType;
pub use repository::{
    BNISoundPreset, EncryptionItem, Item, ItemID, NIContainer, Preset, PresetChunkItem,
    RepositoryRoot,
};
