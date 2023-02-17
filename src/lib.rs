//!
//! NI-FILE
//! Native Instruments file formats
//!
//!  TODO:
//!  - read RepositoryRoot struct
//!  - use radare2 to explore RepositoryRoot
//!

#![warn(clippy::all)]
// #![warn(missing_docs)]
// #![warn(clippy::style)]

#[macro_use]
extern crate log;

pub mod error;
pub mod prelude;

// presets
pub mod preset;

// TODO: make private:

pub mod frame;
pub mod item;

// TODO: delete:

// abstract objects
pub mod ni_object;
pub mod ni_object_data;

// raw objects (know nothing of types, just data containers)
pub mod kinds; // segment type
pub mod raw_data; // data segments (fields)
pub mod raw_repository; // file container / repository

// utils
pub mod cb; // control byte
pub mod deflate; // decompress
pub mod detect; // detect filetype

pub mod extract;

pub(crate) mod read_bytes;
pub(crate) mod utils;

// old
// pub mod raw_container; // old version?
