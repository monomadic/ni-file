#[macro_use]
extern crate log;

pub mod error;
pub mod prelude;

// abstract preset
// pub mod Preset

// abstract container
pub mod container;

pub mod cb; // control byte
pub mod deflate;
pub mod detect;
pub mod ni_container;
pub mod ni_object;
pub mod ni_repository;
pub mod ni_segment;
