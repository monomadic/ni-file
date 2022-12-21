//! NI-FILE
//! native instruments file formats
//!
//!  TODO:
//!  - fix tests
//!

#[macro_use]
extern crate log;

pub mod error;
pub mod prelude;

pub mod ni_object;
// pub mod ni_preset;

// raw objects (know nothing of types, just data containers)
pub mod kinds; // segment type
pub mod raw_data; // data segments (fields)
pub mod raw_repository; // file container / repository

// utils
pub mod cb; // control byte
pub mod deflate; // decompress
pub mod detect; // detect filetype

// old
// pub mod raw_container; // old version?
