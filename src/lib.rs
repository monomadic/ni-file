#[macro_use]
extern crate log;

pub type Error = Box<dyn std::error::Error>;

pub mod cb;
pub mod deflate;
pub mod detect;
pub mod ni_container;
pub mod ni_repository;
pub mod ni_segment;
