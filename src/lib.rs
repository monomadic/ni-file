#[macro_use]
extern crate log;

pub mod error;
pub mod prelude;

// abstract models
// pub mod Preset
pub mod container;

// raw objects
pub mod cb; // control byte
pub mod deflate;
pub mod detect;
pub mod ni_container;
pub mod ni_object;
pub mod ni_repository;
pub mod ni_segment;
