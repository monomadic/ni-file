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

pub mod repository;

// utils
pub mod cb; // control byte
pub mod deflate; // decompress
pub mod detect; // detect filetype

// pub mod extract;

pub(crate) mod read_bytes;
pub(crate) mod utils;

// old
// pub mod raw_container; // old version?
