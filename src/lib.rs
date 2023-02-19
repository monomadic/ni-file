//!
//! NI-FILE
//! Native Instruments file formats
//!
//!  TODO:
//!  - read children
//!  - detect type
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

// TODO: make private:

pub mod item; // rename to repository
pub mod item_frame; // move to repository

// TODO: delete:

// abstract objects
// pub mod ni_object;
// pub mod ni_object_data;

// raw objects (know nothing of types, just data containers)
// pub mod kinds; // segment type
// pub mod raw_data; // data segments (fields)
// pub mod raw_repository; // file container / repository

// utils
pub mod cb; // control byte
pub mod deflate; // decompress
pub mod detect; // detect filetype

// pub mod extract;

pub(crate) mod read_bytes;
pub(crate) mod utils;

// old
// pub mod raw_container; // old version?
