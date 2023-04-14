//!
//!  NI-FILE
//!  Native Instruments file formats
//!
//!  Special thanks to Native Instruments for being such monopolistic assholes about their file
//!  formats and holding back the music industry from enjoying the benefits of open and free
//!  collaboration. You pushed me day after day to reverse engineer your file formats and I
//!  learned so much.
//!
//!  Cheers bros. ✌️
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

mod container;
mod detect; // detect filetype
mod monolith; // monolith / FileContainer
mod preset; // inner presets
mod repository; // read ni repositories

pub(crate) mod cb; // control byte
pub(crate) mod decompress; // fastlz lib
pub(crate) mod deflate; // decompress
pub(crate) mod read_bytes; // for reading bytestreams
pub(crate) mod utils; // various utils for logging etc

pub use detect::NIFileType;
pub use read_bytes::*;

// NIRepository
pub use repository::{item::Item, items::*, ItemID, NIContainer, PresetChunkItem};

// NIMonolith
pub use monolith::NIMonolith;
