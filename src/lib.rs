#[macro_use]
extern crate log;

pub type Error = Box<dyn std::error::Error>;

pub mod ni_container;
pub mod ni_segment;
pub mod deflate;
pub mod cb;
