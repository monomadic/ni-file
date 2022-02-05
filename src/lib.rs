#[macro_use]
extern crate log;

pub type Error = Box<dyn std::error::Error>;

pub mod ni_container;
